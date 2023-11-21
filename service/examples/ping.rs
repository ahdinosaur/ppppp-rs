use std::convert::Infallible;

use ppppp_service::method::{SyncMethod, SyncMethodHandler};

#[derive(Debug)]
struct PingService {}

#[derive(Debug)]
struct Ping {}

#[derive(Debug)]
struct Pong {}

impl SyncMethod<Ping> for PingService {
    type Response = Pong;
    type Error = Infallible;
}

impl SyncMethodHandler<Ping> for PingService {
    fn handle(&mut self, _request: Ping) -> Result<Self::Response, Self::Error> {
        Ok(Pong {})
    }
}

fn main() {
    let mut service = PingService {};

    let request = Ping {};
    let response = service.handle(request);

    println!("response: {:?}", response);
}
