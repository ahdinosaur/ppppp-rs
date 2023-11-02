use std::{error::Error, marker::PhantomData};

pub trait Source: futures_core::Stream {}
pub trait Sink<Item>: futures_sink::Sink<Item> {}
pub struct Duplex<Src: Source, SnkItem, Snk: Sink<SnkItem>> {
    pub source: Src,
    pub sink: Snk,
    sink_item: PhantomData<SnkItem>,
}

pub enum EndState {
    Nil,
    Error(Box<dyn Error>),
    Done,
}
pub trait PullSource<Output, OutputCb: Fn(EndState, Output)>: Fn(EndState, OutputCb) {}
pub trait PullSink<Input, InputCb: Fn(EndState, Input), Src: PullSource<Input, InputCb>>:
    Fn(Src)
{
}

/*
pub type PullSource<Output> = dyn Fn(EndState, dyn Fn(EndState, Output));
*/
