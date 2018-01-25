use errors::*;
use futures::*;

pub trait Runnable {
    fn run(&self) -> Box<Future<Item = (), Error = Error>>;
}
