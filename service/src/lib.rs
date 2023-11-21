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

pub mod method;
pub mod transport;

