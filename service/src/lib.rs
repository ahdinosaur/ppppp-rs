use std::future::Future;

struct MethodName(Vec<String>);

enum MethodType {
    Sync,
    Async,
    Source,
    Sink,
    Duplex,
}

trait SyncMethod<Request> {
    type Context;
    type Response;
    type Error;

    fn call(context: &mut Self::Context, request: Request) -> Result<Self::Response, Self::Error>;
}

trait AsyncMethod<Request> {
    type Context;
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn call(context: &mut Self::Context, request: Request) -> Self::Future;
}

trait SourceMethod<Request> {
    type Context;
    type Output;
    type Error;
    type Source: futures::Stream<Item = Result<Self::Output, Self::Error>>;

    fn call(context: &mut Self::Context, request: Request) -> Self::Source;
}

trait SinkMethod<Request> {
    type Context;
    type Input;
    type Error;
    type Sink: futures::Sink<Self::Input, Error = Self::Error>;

    fn call(context: &mut Self::Context, request: Request) -> Self::Sink;
}

trait DuplexMethod<Request> {
    type Context;
    type Input;
    type Output;
    type Error;
    type Source: futures::Stream<Item = Result<Self::Output, Self::Error>>;
    type Sink: futures::Sink<Self::Input, Error = Self::Error>;

    fn call(context: &mut Self::Context, request: Request) -> (Self::Source, Self::Sink);
}

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
