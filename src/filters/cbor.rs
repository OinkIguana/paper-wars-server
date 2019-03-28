use serde::Serialize;
use serde_cbor::to_vec;
use warp::http::Response;

pub fn cbor<T: Serialize>(value: &T) -> Response<Vec<u8>> {
    Response::new(to_vec(value).unwrap())
}
