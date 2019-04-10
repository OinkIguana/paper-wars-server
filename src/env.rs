use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

lazy_static! {
    pub static ref SCHEMA_DIR: PathBuf = PathBuf::from(env::var("SCHEMA_DIR").unwrap_or("../schema/".to_string()));
    pub static ref MAKER_DIR: PathBuf = PathBuf::from(env::var("MAKER_DIR").unwrap_or("../frontend-maker/dist/".to_string()));
    pub static ref PLAYER_DIR: PathBuf = PathBuf::from(env::var("PLAYER_DIR").unwrap_or("../frontend-player/dist/".to_string()));
    pub static ref PORT: u16 = env::var("PORT").ok().and_then(|port| port.parse().ok()).unwrap_or(15320);
}
