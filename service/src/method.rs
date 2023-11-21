use std::future::Future;

pub enum MethodType {
    Sync,
    Async,
    Source,
    Sink,
    Duplex,
}

/*
pub trait Method<Request> {
    const NAME: &'static [&'static str];
    const TYPE: MethodType;
}
*/

pub trait SyncMethod<Request> {
    type Response;
    type Error;
}

pub trait SyncMethodHandler<Request>: SyncMethod<Request> {
    fn handle(&mut self, request: Request) -> Result<Self::Response, Self::Error>;
}

pub trait AsyncMethod<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
}

pub trait AsyncMethodHandler<Request>: AsyncMethod<Request> {
    fn handle(&mut self, request: Request) -> Self::Future;
}

pub trait SourceMethod<Request> {
    type Output;
    type Error;
    type Source: futures::Stream<Item = Result<Self::Output, Self::Error>>;
}

pub trait SourceMethodHandler<Request>: SourceMethod<Request> {
    fn handle(&mut self, request: Request) -> Self::Source;
}

pub trait SinkMethod<Request> {
    type Input;
    type Error;
    type Sink: futures::Sink<Self::Input, Error = Self::Error>;
}

pub trait SinkMethodHandler<Request>: SinkMethod<Request> {
    fn handle(&mut self, request: Request) -> Self::Sink;
}

pub trait DuplexMethod<Request> {
    type Input;
    type Output;
    type Error;
    type Source: futures::Stream<Item = Result<Self::Output, Self::Error>>;
    type Sink: futures::Sink<Self::Input, Error = Self::Error>;
}

pub trait DuplexMethodHandler<Request>: DuplexMethod<Request> {
    fn handle(&mut self, request: Request) -> (Self::Source, Self::Sink);
}
