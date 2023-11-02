use napi::bindgen_prelude::*;

use crate::SourceStream;

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

fn to_pull_source<
    ReadData,
    ReadError,
    Src: SourceStream<Item = Result<ReadData, ReadError>>,
    ReadCb: Fn(End<ReadError>, ReadData),
>(
    source: Src,
) -> impl PullSource<Error, ReadData, ReadError, ReadCb>
where
    ReadError: AsRef<str>,
{
    |end, cb| {
        if let Some(EndState::Error(err)) | Some(EndState::Done) = end {
            return cb(end);
        };
        cb(None, 0);
    }
}
