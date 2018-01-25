//! Snoop plugin that exposes metrics on a local socket.

use errors::*;
use plugin::*;
use metric::*;

use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::net::SocketAddr;
use tokio_io::{io, AsyncRead};
use tokio_core::net;
use futures::{sync, Future};
use futures::stream::Stream;
use std::convert::AsRef;
use std::fmt;
use serde_json;

#[derive(Deserialize, Debug)]
struct SnoopInputConfig {
    bind: Option<SocketAddr>,
}

#[derive(Debug)]
struct SnoopOutput {}

fn report_and_discard<E: fmt::Display>(e: E) -> () {
    info!("An error occured: {}", e);
}

type Sender = sync::mpsc::UnboundedSender<Message>;

impl Output for SnoopOutput {
    fn setup(&self, ctx: PluginContext) -> Result<Box<OutputInstance>> {
        let config: SnoopInputConfig = ctx.decode_config()?;

        let ref mut core = ctx.core.try_borrow_mut()?;

        let default_addr = "127.0.0.1:8080".parse::<SocketAddr>().map_err(|e| {
            ErrorKind::Message(e.to_string())
        })?;

        let addr = config.bind.unwrap_or(default_addr);

        let handle = core.handle();

        let socket = net::TcpListener::bind(&addr, &handle)?;

        let connections: Arc<Mutex<HashMap<SocketAddr, Sender>>> =
            Arc::new(Mutex::new(HashMap::new()));

        let hello_connections = connections.clone();

        let accept = socket.incoming().map_err(report_and_discard).for_each(
            move |(socket, addr)| {
                info!("connect: {}", addr);

                let (tx, rx) = sync::mpsc::unbounded();

                let (reader, writer) = socket.split();

                hello_connections
                    .lock()
                    .map_err(report_and_discard)?
                    .insert(addr, tx);

                let socket_writer = rx.fold(writer, |writer, msg| {
                    let amt = io::write_all(writer, msg);
                    amt.map(|(writer, _m)| writer).map_err(report_and_discard)
                }).map(|_| ());

                let bye_connections = hello_connections.clone();

                // read one byte, bur more importantly the future will be notified on disconnects.
                // TODO: this has the side-effect that anything written by the client will cause it
                // to be immediately disconnected.
                let reader = io::read_exact(reader, vec![0; 1]).map(|_| ()).map_err(
                    report_and_discard,
                );

                let conn = reader.select(socket_writer);

                handle.spawn(conn.then(move |_| {
                    info!("disconnect: {}", addr);
                    bye_connections.lock().map_err(report_and_discard)?.remove(
                        &addr,
                    );
                    Ok(())
                }));

                Ok(())
            },
        );

        core.handle().spawn(accept);

        Ok(Box::new(SnoopOutputInstance {
            id: ctx.id.clone(),
            connections: connections.clone(),
        }))
    }
}

struct SnoopOutputInstance {
    id: String,
    connections: Arc<Mutex<HashMap<SocketAddr, Sender>>>,
}

#[derive(Serialize)]
struct SerializedOutput<'a> {
    plugin_id: &'a String,
    metric_id: &'a MetricId,
    value: &'a f64,
}

impl OutputInstance for SnoopOutputInstance {
    fn feed(&self, sample: &Sample) -> Result<()> {
        let mut c = self.connections.lock()?;

        // serialize a single output message
        let bytes = {
            let serialized = SerializedOutput {
                plugin_id: &self.id,
                metric_id: &sample.metric_id,
                value: &sample.value,
            };

            let mut bytes: Vec<u8> = serde_json::to_string(&serialized)?.into_bytes();
            bytes.push('\n' as u8);
            bytes
        };

        // put in ref-counted data structure to avoid copying to all connections.
        let out = Message(Arc::new(bytes.into_boxed_slice()));

        for (_addr, tx) in c.iter_mut() {
            let _r = tx.unbounded_send(out.clone()).map_err(|_| {
                ErrorKind::Message("failed to feed".to_owned())
            })?;
        }

        Ok(())
    }
}

pub fn output() -> Result<Box<Output>> {
    Ok(Box::new(SnoopOutput {}))
}

#[derive(Clone)]
pub struct Message(Arc<Box<[u8]>>);

impl AsRef<[u8]> for Message {
    fn as_ref(&self) -> &[u8] {
        let &Message(ref a) = self;
        &a
    }
}
