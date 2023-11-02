use futures::StreamExt;
use napi::bindgen_prelude::{spawn, Error};

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
type PullSource<Value> = Box<dyn FnMut(End, Box<dyn Fn(End, Option<Value>)>)>;

fn to_pull_source<Value, Src: Source<Item = Result<Value, Error>> + Unpin + Send + 'static>(
    mut source: Src,
) -> PullSource<Value> {
    Box::new(move |end, cb| {
        let source = &mut source;
        spawn(async {
            match source.next().await {
                Some(Ok(value)) => cb(None, Some(value)),
                Some(Err(err)) => cb(Some(EndState::Error(err)), None),
                None => cb(Some(EndState::Done), None),
            }
        });
    })
}
