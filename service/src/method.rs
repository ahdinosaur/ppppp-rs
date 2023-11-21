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

pub trait SyncMethodHandler<Service: SyncMethod<Request>, Request> {
    fn handle(&mut self, request: Request) -> Result<Service::Response, Service::Error>;
}

pub trait AsyncMethod<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
}

pub trait AsyncMethodHandler<Service: AsyncMethod<Request>, Request> {
    fn handle(&mut self, request: Request) -> Service::Future;
}

pub trait SourceMethod<Request> {
    type Output;
    type Error;
    type Source: futures::Stream<Item = Result<Self::Output, Self::Error>>;
}

pub trait SourceMethodHandler<Service: SourceMethod<Request>, Request> {
    fn handle(&mut self, request: Request) -> Service::Source;
}

pub trait SinkMethod<Request> {
    type Input;
    type Error;
    type Sink: futures::Sink<Self::Input, Error = Self::Error>;
}

pub trait SinkMethodHandler<Service: SinkMethod<Request>, Request> {
    fn handle(&mut self, request: Request) -> Service::Sink;
}

pub trait DuplexMethod<Request> {
    type Input;
    type Output;
    type Error;
    type Source: futures::Stream<Item = Result<Self::Output, Self::Error>>;
    type Sink: futures::Sink<Self::Input, Error = Self::Error>;
}

pub trait DuplexMethodHandler<Service: DuplexMethod<Request>, Request> {
    fn handle(&mut self, request: Request) -> (Service::Source, Service::Sink);
}
