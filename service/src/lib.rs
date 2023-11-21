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

use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt::Debug};
use transport::{Connection, ServerEndpoint};

pub mod method;
pub mod transport;

/// Requirements for a [Service] message
///
/// Even when just using the in-memory transport, we require messages to be Serializable and Deserializable.
/// Likewise, even when using the network transport, we require messages to be Send.
pub trait RpcMessage: Debug + Serialize + DeserializeOwned + Send + Sync + Unpin + 'static {}

impl<T> RpcMessage for T where
    T: Debug + Serialize + DeserializeOwned + Send + Sync + Unpin + 'static
{
}

/// Requirements for an internal error
///
/// All errors have to be Send, Sync and 'static so they can be sent across threads.
/// They also have to be Error (which includes Debug and Display) so they can be logged.
pub trait RpcError: Error + Send + Sync + Unpin + 'static {}

impl<T> RpcError for T where T: Error + Send + Sync + Unpin + 'static {}

/// A service
///
/// A service has request and response message types. These types have to be the
/// union of all possible request and response types for all interactions with
/// the service.
///
/// Usually you will define an enum for the request and response
/// type, and use the [derive_more](https://crates.io/crates/derive_more) crate to
/// define the conversions between the enum and the actual request and response types.
///
/// To make a message type usable as a request for a service, implement a [method] for
/// it.
///
/// A message type can be used for multiple services. E.g. you might have a
/// Status request that is understood by multiple services and returns a
/// standard status response.
pub trait Service: Send + Sync + Debug + Clone + 'static {
    type Request: RpcMessage;
    type Response: RpcMessage;
}

/// A connection to a specific service on a specific remote machine
///
/// This is just a trait alias for a [Connection] with the right types.
///
/// This can be used to create a [RpcClient] that can be used to send requests.
pub trait ServiceConnection<S: Service>: Connection<S::Response, S::Request> {}

impl<T: Connection<S::Response, S::Request>, S: Service> ServiceConnection<S> for T {}

/// A server endpoint for a specific service
///
/// This is just a trait alias for a [ServerEndpoint] with the right types.
///
/// This can be used to create a [RpcServer] that can be used to handle
/// requests.
pub trait ServiceEndpoint<S: Service>: ServerEndpoint<S::Request, S::Response> {}

impl<T: ServerEndpoint<S::Request, S::Response>, S: Service> ServiceEndpoint<S> for T {}
