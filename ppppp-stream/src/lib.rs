use std::{error::Error, marker::PhantomData};

pub trait Source: futures_core::Stream {}
pub trait Sink<Item>: futures_sink::Sink<Item> {}
pub struct Duplex<Src: Source, SnkItem, Snk: Sink<SnkItem>> {
    pub source: Src,
    pub sink: Snk,
    sink_item: PhantomData<SnkItem>,
}

pub enum EndState<Error> {
    Nil,
    Error(Error),
    Done,
}
pub trait PullSource<ReaderError, Output, ReadError, OutputCb: Fn(EndState<ReadError>, Output)>: Fn(EndState<ReaderError>, OutputCb) {}
pub trait PullSink<ReaderError, Input, ReadError, InputCb: Fn(EndState<ReaderError>, Input), Src: PullSource<ReaderError, Input, ReadError, InputCb>>:
    Fn(Src)
{
}

fn to_pull_source<Output, ReadError, Src: Source<Item = Output>>(source: Src): PullSource<Box<dyn Error>, Output, ReadError > {

}

/*
pub type PullSource<Output> = dyn Fn(EndState, dyn Fn(EndState, Output));
*/
