use futures::{pin_mut, StreamExt};
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
type PullSource<Value> = Box<dyn Fn(End, Box<dyn Fn(End, Option<Value>) + Send>)>;

fn to_pull_source<
    Value,
    Src: Source<Item = Result<Value, Error>> + Unpin + Send + Sync + 'static,
>(
    source: Src,
) -> PullSource<Value> {
    Box::new(move |end, cb| {
        if let Some(end_state) = end {
            match end_state {
                EndState::Error(err) => cb(Some(EndState::Error(err)), None),
                EndState::Done => cb(Some(EndState::Done), None),
            };
            return;
        }

        pin_mut!(source);

        spawn(async move {
            match source.next().await {
                Some(Ok(value)) => cb(None, Some(value)),
                Some(Err(err)) => cb(Some(EndState::Error(err)), None),
                None => cb(Some(EndState::Done), None),
            }
        });
    })
}
