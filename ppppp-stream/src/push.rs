use napi::bindgen_prelude::*;

use crate::{Sink, Source};

enum EndState {
    Error(Error),
    Done,
}

// pub trait SourceStream: futures_core::Stream {}
// pub trait SinkStream<Item>: futures_sink::Sink<Item> {}

pub struct PushSource<Value> {
    source: Box<dyn Source<Item = Result<Value, Error>>>,
}

impl<Value> PushSource<Value> {
    fn new(source: impl Source<Item = Result<Value, Error>>) {}
    fn pipe(sink: PushSink<Value>) {}
    fn resume() {}
    fn abort() {}
}

pub struct PushSink<Value> {
    sink: Box<dyn Sink<Value, Error = Error>>,
}

impl<Value> PushSink<Value> {
    fn new(sink: impl Sink<Value, Error = Error>) {}
    fn write(value: Value) {}
    fn end() {}
    fn abort() {}
}
