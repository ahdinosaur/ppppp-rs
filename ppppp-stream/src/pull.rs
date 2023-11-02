use futures::StreamExt;
use napi::bindgen_prelude::Error;

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
type PullSource<Value> = dyn Fn(End, dyn Fn(End, Option<Value>));

fn to_pull_source<Value, Src: Source<Item = Result<Value, Error>>>(
    source: Src,
) -> PullSource<Value> {
    |end, cb| {
        tokio::spawn(async {
            match source.next().await {
                Some(Ok(value)) => cb(None, value),
                Some(Err(err)) => cb(Some(EndState::Error(err)), None),
                None => cb(Some(EndState::Done), None),
            }
        })
    }
}
