//! Snoop plugin that exposes metrics on a local socket.

use ::errors::*;
use ::plugin::*;

use std::net::SocketAddr;
use tokio_core::io::{copy, Io};
use tokio_core::net::TcpListener;
use futures::Future;
use futures::stream::Stream;

#[derive(Deserialize, Debug)]
struct SnoopInputConfig {
    target: String,
}

#[derive(Debug)]
struct SnoopOutput {
}

impl Output for SnoopOutput {
    fn setup(&self, ctx: PluginContext) -> Result<Box<OutputInstance>> {
        let ref mut core = ctx.core.try_borrow_mut()?;

        let addr = "127.0.0.1:8080".parse::<SocketAddr>()
            .map_err(|e| ErrorKind::Message(e.to_string()))?;
        let handle = core.handle();

        let socket = TcpListener::bind(&addr, &handle)?;

        let done = socket.incoming()
            .map_err(|_| ())
            .for_each(move |(socket, addr)| {
                let (reader, writer) = socket.split();
                let amt = copy(reader, writer);

                let msg = amt.then(move |result| {
                    match result {
                        Ok(amt) => println!("wrote {} bytes to {}", amt, addr),
                        Err(e) => println!("error on {}: {}", addr, e),
                    }

                    Ok(())
                });

                handle.spawn(msg);

                Ok(())
            });

        core.handle().spawn(done);

        Ok(Box::new(SnoopOutputInstance::new(ctx.id.clone())))
    }
}

#[derive(Debug)]
struct SnoopOutputInstance {
    id: String,
}

impl SnoopOutputInstance {
    pub fn new(id: String) -> SnoopOutputInstance {
        SnoopOutputInstance { id: id }
    }
}

impl OutputInstance for SnoopOutputInstance {
    fn feed(&self, sample: &Sample) {
        info!("  debug: {:?} {:?}", self.id, sample.metric_id);
        info!("      => {}", sample.value);
    }
}

pub fn output() -> Result<Box<Output>> {
    Ok(Box::new(SnoopOutput {}))
}
