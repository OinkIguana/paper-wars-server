use std::fs;
use std::path::{Path, PathBuf};
use toml;
use ron;
use serde::{Serialize, Deserialize};
use warp::{Rejection, reject::{custom, not_found}};

mod game;
mod localization;
mod universe;
pub use game::*;
pub use localization::*;
pub use universe::*;

fn write_ron<T, P>(path: P, data: &T) -> Result<(), Rejection>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let string = ron::ser::to_string(data).map_err(custom)?;
    if path.as_ref().exists() {
        return Err(custom("Game already exists"));
    }
    fs::create_dir_all(path.as_ref().parent().unwrap())
        .map_err(custom)?;
    fs::write(path, &string).map_err(custom)?;
    Ok(())
}

fn parse_ron<T, P>(path: P) -> Result<T, Rejection>
where
    for<'de> T: Deserialize<'de>,
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(path.as_ref()).map_err(custom)?;
    ron::de::from_str(&contents).map_err(custom)
}

fn parse_toml<T, P>(path: P) -> Result<T, Rejection>
where
    for<'de> T: Deserialize<'de>,
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(path.as_ref()).map_err(custom)?;
    toml::from_str(&contents).map_err(custom)
}

pub fn load_directory<T, E, P, F>(path: P, loader: F) -> Result<impl Iterator<Item = T>, Rejection>
where
    P: AsRef<Path>,
    F: FnMut(PathBuf) -> Result<T, E>,
{
    Ok(fs::read_dir(path.as_ref())
        .map_err(|_| not_found())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .map(loader)
        .filter_map(Result::ok))
}
