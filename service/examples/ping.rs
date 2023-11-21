use derive_more::{From, TryInto};
use ppppp_service::{
    method::{SyncMethod, SyncMethodHandler},
    Service,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Clone)]
struct PingService;

impl Service for PingService {
    type Request = PingServiceRequest;
    type Response = PingServiceResponse;
}

#[derive(Debug, Serialize, Deserialize)]
struct Ping;

#[derive(Debug, Serialize, Deserialize, From, TryInto)]
enum PingServiceRequest {
    Ping(Ping),
}

#[derive(Debug, Serialize, Deserialize)]
struct Pong;

#[derive(Debug, Serialize, Deserialize, From, TryInto)]
enum PingServiceResponse {
    Pong(Pong),
}

impl SyncMethod<Ping> for PingService {
    type Response = Pong;
    type Error = Infallible;
}

#[derive(Debug)]
struct PingServiceHandler;

impl SyncMethodHandler<PingService, Ping> for PingServiceHandler {
    fn handle(&mut self, _request: Ping) -> Result<Pong, Infallible> {
        Ok(Pong {})
    }
}

fn main() {
    let mut handler = PingServiceHandler {};

    let request = Ping {};
    let response = handler.handle(request);

    println!("response: {:?}", response);
}
