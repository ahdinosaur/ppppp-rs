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
