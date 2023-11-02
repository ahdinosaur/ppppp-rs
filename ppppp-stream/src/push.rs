use napi::bindgen_prelude::Error as JsError;

use crate::{Sink, Source};

enum EndState {
    Error(JsError),
    Done,
}

// pub trait SourceStream: futures_core::Stream {}
// pub trait SinkStream<Item>: futures_sink::Sink<Item> {}

pub struct PushSource<Value, Error, Src>
where
    Src: Source<Value = Result<Value, Error>>,
{
    source: Src,
}

impl<Value, Error, Src> PushSource<Value, Error, Src>
where
    Src: Source<Value = Result<Value, Error>>,
{
    fn new(source: Src) {}
    fn pipe(sink: PushSink<Value = Result<Value, JsError>>) {}
    fn resume() {}
    fn abort() {}
}

pub struct PushSink<Value, Error, Snk>
where
    Snk: Sink<Result<Value, Error>>,
{
    sink: Snk,
}

impl<Value, Error, Snk> PushSink<Value, Error, Snk>
where
    Snk: Sink<Result<Value, Error>>,
{
    fn new(sink: Snk) {}
    fn write(value: Value) {}
    fn end() {}
    fn abort() {}
}
