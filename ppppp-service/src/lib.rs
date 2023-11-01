use std::{error::Error, future::Future};

use async_trait::async_trait;
use tokio_stream::Stream;

/*
type SourceStream<Output> = dyn Stream<Item = Output>;
type SinkStream<Input, SinkError> =
    fn(dyn Stream<Item = Input>) -> dyn Future<Output = Result<(), SinkError>>;
type DuplexStream<Output, Input, SinkError> = (SourceStream<Output>, SinkStream<Input, SinkError>);
*/

/*
pub enum ServiceData<Value> {
    Sync(Value),
    Async(Box<dyn Future<Output = Value>>),
    Source(Box<dyn Stream<Item = Value>>),
    Sink(fn(Box<dyn Stream<Item = Value>>) -> dyn Future<Output = ()>),
}
*/

pub enum ServiceDataType {
    Sync,
    Async,
    Source,
    Sink,
}

pub trait SyncServiceMethod {
    type Name: ToString;
    type Request;
    type Error: Error;
    type Response;

    fn call(request: Self::Request) -> Result<Self::Response, Self::Error>;
}

pub trait AsyncServiceMethod {
    type Name: ToString;
    type Request;
    type Error: Error;
    type Response;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
}

pub trait SourceServiceMethod {
    type Name: ToString;
    type Request;
    type Error: Error;
    type SourceOutput;
    type SourceStream: Stream<Item = Result<Self::SourceOutput, Self::Error>>;
}

pub trait SinkServiceMethod {
    type Name: ToString;
    type Request;
    type Error;
    type Future: Future<Output = Result<(), Self::Error>>;
    type SinkInput;
    type SinkInputStream: Stream<Item = Self::SinkInput>;
    type SinkStream: Fn(Self::SinkInputStream) -> Self::Future;
}

pub trait DuplexServiceMethod {
    type Name: ToString;
    type Request;
    type SourceOutput;
    type SourceError;
    type SourceStream: Stream<Item = Result<Self::SourceOutput, Self::SourceError>>;
    type SinkError;
    type SinkInput;
    type SinkFuture: Future<Output = Result<(), Self::SinkError>>;
    type SinkInputStream: Stream<Item = Self::SinkInput>;
    type SinkStream: Fn(Self::SinkInputStream) -> Self::SinkFuture;
}

pub struct Service<Manifest> {
    name: String,
    version: String,
    manifest: Manifest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
