struct MethodName(Vec<String>);

enum MethodType {
    Sync,
    Async,
    Source,
    Sink,
    Duplex,
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

// then have a transport trait for each of those traits
//   where each transport is given a connection duplex stream.
//
// ... hmm... but the routing needs to happen at the top.

// sync transport
// async transport
// source transport
// sink transport
// duplex transport
