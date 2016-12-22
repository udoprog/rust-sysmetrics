use ::errors::*;
use futures::*;

pub trait Runnable {
    fn run(&self) -> BoxFuture<(), Error>;
}
