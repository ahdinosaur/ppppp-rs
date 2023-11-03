use std::fmt::Debug;

use futures::{pin_mut, StreamExt};
use napi::bindgen_prelude::{spawn, Error};
use pin_utils::unsafe_pinned;

use crate::Source;

/*
pub enum EndState<Error> {
    Error(Error),
    Done,
}
pub type End<Error> = Option<EndState<Error>>;
pub trait PullSource<ReaderError, ReadData, ReadError, ReadCb: Fn(End<ReadError>, Option<ReadData>)>:
    Fn(End<ReaderError>, ReadCb)
{
}
pub trait PullSink<
    ReaderError,
    WriteData,
    ReadError,
    ReadCb: Fn(End<ReadError>, WriteData),
    Src: PullSource<ReaderError, WriteData, ReadError, ReadCb>,
>: Fn(Src)
{
}
*/

pub enum EndState {
    Error(Error),
    Done,
}
pub type End = Option<EndState>;

pub struct PullSource<Src> {
    source: Src,
}

impl<Src: Unpin> Unpin for PullSource<Src> {}

impl<Src> PullSource<Src>
where
    Src: Source,
{
    unsafe_pinned!(source: Src);

    pub fn new(source: Src) -> Self {
        Self { source }
    }
}

impl<Src, Value> PullSource<Src>
where
    Src: Source<Item = Result<Value, Error>> + Send + 'static,
    Value: Debug,
{
    fn read<Cb>(&self, end: End, cb: Cb)
    where
        Cb: Fn(End, Option<Value>) + Unpin + Send + 'static,
    {
        let mut source = Box::pin(self.source);

        if let Some(end_state) = end {
            match end_state {
                EndState::Error(err) => cb(Some(EndState::Error(err)), None),
                EndState::Done => cb(Some(EndState::Done), None),
            };
            return;
        }

        spawn(async move {
            match source.next().await {
                Some(Ok(value)) => cb(None, Some(value)),
                Some(Err(err)) => cb(Some(EndState::Error(err)), None),
                None => cb(Some(EndState::Done), None),
            }
        });
    }
}

/*
type PullSourceFn<Value> = Box<dyn Fn(End, Box<dyn Fn(End, Option<Value>) + Send>)>;

fn to_pull_source<Value, Src>(source: Src) -> PullSourceFn<Value>
where
    Value: Debug,
    Src: Source<Item = Result<Value, Error>> + Send + 'static,
{
    let pull_source = PullSource::new(source);
    Box::new(move |end, cb| pull_source.read(end, cb))
}
*/
