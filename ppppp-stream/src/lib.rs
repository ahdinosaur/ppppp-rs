// - https://docs.rs/tokio-util/latest/tokio_util/io/index.html
// - https://github.com/napi-rs/napi-rs/tree/main/examples/napi
// - https://github.com/MattiasBuelens/wasm-streams/blob/main/src/readable/mod.rs
//
//
// NOTES:
//
// - two types of streams: object streams and buffer streams
// - four types of streams: source, sink, through, and duplex
//
// - maybe start where JS interop is in JavaScript? so we just export Rust stream interfaces.
//
use std::marker::PhantomData;

mod pull;
mod push;

pub trait Source: futures::Stream {}
pub trait Sink<Item>: futures::Sink<Item> {}
pub struct Duplex<Src: Source, SnkItem, Snk: Sink<SnkItem>> {
    pub source: Src,
    pub sink: Snk,
    sink_item: PhantomData<SnkItem>,
}
