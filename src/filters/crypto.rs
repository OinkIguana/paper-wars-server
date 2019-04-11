use serde::Deserialize;
use warp::reject::{Rejection, custom};
use shared::crypto::Signed;

pub fn verify<T: for<'de> Deserialize<'de>>(signed: Signed<T>) -> Result<T, Rejection> {
    signed
        .verify()
        .map_err(|_| custom("Unauthorized"))
}
