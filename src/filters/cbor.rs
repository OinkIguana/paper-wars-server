use bytes::Buf;
use serde::{Serialize, Deserialize};
use serde_cbor::{to_vec, from_reader};
use warp::http::Response;
use warp::body::FullBody;
use warp::reject::custom;
use warp::Rejection;

pub fn cbor<T: Serialize>(value: &T) -> Response<Vec<u8>> {
    Response::new(to_vec(value).unwrap())
}

pub fn from_cbor<T: for<'de> Deserialize<'de>>(value: FullBody) -> Result<T, Rejection> {
    from_reader(value.reader()).map_err(custom)
}
