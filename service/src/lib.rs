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
//
//
// hmmm....
// ... how do we handle the trait's associated types?
// ... there's not really any way to register methods with such generic types.
//
// looking at tarpc, should we be thinking of singular service structs, with functions attached.
//   if one service will call another service, they probably will be separate anyways.
//   one service can get a client connection to another service.
//   rather than in SSB where everything is by default able to touch anything else.
//
// looking at quic-rpc, i notice:
//   - the server handler routing is done manually, not automatically. so that'd help.
//   - and same with the client calls.
// so maybe we can do something similar:
//
// match msg {
//   Ping(args) => {
//      server.handle_async(Ping, args)
//   }
// }
//
// the main challenge here is that we need a single type of represent incoming messages.
// but muxrpc has a weird (for Rust) structure:
//
// {
//  "name": ["createHistoryStream"],
//  "type": "source",
//  "args": [{"id": "@FCX/tsDLpubCPKKfIrw4gc+SQkHcaD17s7GI6i/ziWY=.ed25519"}]
// }
//
// maybe we just do the JSON + MuxRPC part separately.
//   like a name + type -> deserialized args struct registry.
//   where we choose which methods to expose over MuxRPC.
//   then we can convert these args structs into the top-level enum.
//
//

use std::{convert::Infallible, future::Future};

pub enum MethodType {
    Sync,
    Async,
    Source,
    Sink,
    Duplex,
}

pub trait Method<Request> {
    const NAME: &'static [&'static str];
    const TYPE: MethodType;
}

pub trait SyncMethod<Request> {
    type Response;
    type Error;

    fn call(&mut self, request: Request) -> Result<Self::Response, Self::Error>;
}

pub trait AsyncMethod<Request> {
    const NAME: &'static [&'static str];

    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn call(&mut self, request: Request) -> Self::Future;
}

pub trait SourceMethod<Request> {
    const NAME: &'static [&'static str];

    type Output;
    type Error;
    type Source: futures::Stream<Item = Result<Self::Output, Self::Error>>;

    fn call(&mut self, request: Request) -> Self::Source;
}

pub trait SinkMethod<Request> {
    const NAME: &'static [&'static str];

    type Input;
    type Error;
    type Sink: futures::Sink<Self::Input, Error = Self::Error>;

    fn call(&mut self, request: Request) -> Self::Sink;
}

pub trait DuplexMethod<Request> {
    const NAME: &'static [&'static str];

    type Input;
    type Output;
    type Error;
    type Source: futures::Stream<Item = Result<Self::Output, Self::Error>>;
    type Sink: futures::Sink<Self::Input, Error = Self::Error>;

    fn call(&mut self, request: Request) -> (Self::Source, Self::Sink);
}

// ---

struct PingService {}

struct Ping {}
struct Pong {}

impl Method<Ping> for PingService {
    const NAME: &'static [&'static str] = &["ping"];
    const TYPE: MethodType = MethodType::Sync;
}

impl SyncMethod<Ping> for PingService {
    type Response = Pong;
    type Error = Infallible;

    fn call(&mut self, _request: Ping) -> Result<Self::Response, Self::Error> {
        Ok(Pong {})
    }
}

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
