use serde::Serialize;
use serde_cbor::to_vec;
use warp::http::Response;
use log::debug;

pub fn cbor<T: Serialize + std::fmt::Debug>(value: &T) -> Response<Vec<u8>> {
    debug!("Sending: {:?}", value);
    Response::new(to_vec(value).unwrap())
}
