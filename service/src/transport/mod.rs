use futures::{Future, Sink, Stream};
use std::{
    fmt::{self, Debug, Display},
    net::SocketAddr,
};

use crate::RpcError;

/// Errors that can happen when creating and using a [`Connection`] or [`ServerEndpoint`].
pub trait ConnectionErrors: Debug + Clone + Send + Sync + 'static {
    /// Error when opening or accepting a channel
    type OpenError: RpcError;
    /// Error when sending a message via a channel
    type SendError: RpcError;
    /// Error when receiving a message via a channel
    type RecvError: RpcError;
}

/// Types that are common to both [`Connection`] and [`ServerEndpoint`].
///
/// Having this as a separate trait is useful when writing generic code that works with both.
pub trait ConnectionCommon<In, Out>: ConnectionErrors {
    /// Receive side of a duplex typed channel
    type RecvStream: Stream<Item = Result<In, Self::RecvError>> + Send + Unpin + 'static;
    /// Send side of a duplex typed channel
    type SendSink: Sink<Out, Error = Self::SendError> + Send + Unpin + 'static;
}

/// A connection to a specific remote machine
///
/// A connection can be used to open duplex typed channels using [`Connection::open`].
pub trait Connection<In, Out>: ConnectionCommon<In, Out> {
    /// The future that will resolve to a substream or an error
    type OpenFuture: Future<Output = Result<(Self::SendSink, Self::RecvStream), Self::OpenError>>
        + Send;
    /// Open a channel to the remote
    fn open(&self) -> Self::OpenFuture;
}

/// A server endpoint that listens for connections
///
/// A server endpoint can be used to accept duplex typed channels from any of the
/// currently opened connections to clients, using [`ServerEndpoint::accept`].
pub trait ServerEndpoint<In, Out>: ConnectionCommon<In, Out> {
    /// The future that will resolve to a substream or an error
    type AcceptFuture: Future<Output = Result<(Self::SendSink, Self::RecvStream), Self::OpenError>>
        + Send;

    /// Accept a new typed duplex channel on any of the connections we
    /// have currently opened.
    fn accept(&self) -> Self::AcceptFuture;

    /// The local addresses this endpoint is bound to.
    fn local_addr(&self) -> &[LocalAddr];
}

/// The kinds of local addresses a [ServerEndpoint] can be bound to.
///
/// Returned by [ServerEndpoint::local_addr].
///
/// [`Display`]: fmt::Display
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum LocalAddr {
    /// A local network socket.
    Net(SocketAddr),
    /// An in-memory address.
    Mem,
}

impl Display for LocalAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LocalAddr::Net(sockaddr) => write!(f, "{sockaddr}"),
            LocalAddr::Mem => write!(f, "mem"),
        }
    }
}
