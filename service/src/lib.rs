// have a trait for each type of method

// sync method
// async method
// source method
// sink method
// duplex method

// a service is a collection of methods registered at names.

// then have a transport trait,
//    where a transport is given a service and a duplex connection,
//    and must route to and from the connection and service methods.

use std::future::Future;

pub struct Context {}

pub struct Service {
    context: Context,
}

impl Service {
    pub fn call_sync<Request, Method>(
        &mut self,
        request: Request,
    ) -> Result<Method::Response, Method::Error>
    where
        Method: SyncMethod<Context, Request>,
    {
        Method::call(&mut self.context, request)
    }
}

pub enum MethodType {
    Sync,
    Async,
    Source,
    Sink,
    Duplex,
}

pub trait Method {
    fn get_path() -> &'static [&'static str];
    fn get_type() -> &'static MethodType;
}

pub trait SyncMethod<Context, Request> {
    type Response;
    type Error;

    fn call(context: &mut Context, request: Request) -> Result<Self::Response, Self::Error>;
}

pub trait AsyncMethod<Context, Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn call(context: &mut Context, request: Request) -> Self::Future;
}

pub trait SourceMethod<Context, Request> {
    type Output;
    type Error;
    type Source: futures::Stream<Item = Result<Self::Output, Self::Error>>;

    fn call(context: &mut Context, request: Request) -> Self::Source;
}

pub trait SinkMethod<Context, Request> {
    type Input;
    type Error;
    type Sink: futures::Sink<Self::Input, Error = Self::Error>;

    fn call(context: &mut Context, request: Request) -> Self::Sink;
}

pub trait DuplexMethod<Context, Request> {
    type Input;
    type Output;
    type Error;
    type Source: futures::Stream<Item = Result<Self::Output, Self::Error>>;
    type Sink: futures::Sink<Self::Input, Error = Self::Error>;

    fn call(context: &mut Context, request: Request) -> (Self::Source, Self::Sink);
}

// ---

/*
struct MethodName(Vec<String>);

struct MethodCall<MethodArgs> {
    method_name: MethodName,
    method_type: MethodType,
    method_args: MethodArgs,
}

struct RequestId(i32);
struct BodyLength(u32);

enum BodyType {
    Binary,
    Utf8String,
    Json,
}

enum EndErrorFlag {
    NotEndOrError,
    IsEndOrError,
}

enum StreamFlag {
    NotStream,
    IsStream,
}

struct Packet {
    request_id: i32,
    body_length: u32,
    body_type: BodyType,
    end_error: EndErrorFlag,
    stream: StreamFlag,
}
*/
