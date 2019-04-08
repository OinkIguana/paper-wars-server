use std::ffi::OsStr;
use std::{fs, path::{Path, PathBuf}};
use lazy_static::lazy_static;
use log::warn;
use shared::*;
use crate::env;

lazy_static! {
    static ref FTL_EXT: &'static OsStr = OsStr::new("ftl");
}

pub fn load_localization<S: AsRef<str>>(id: Id<Universe>, language: S) -> String {
    find_localization(&*env::SCHEMA_DIR.join("universes").join(&id), language.as_ref())
        .map(fs::read_to_string)
        .filter_map(Result::ok)
        .collect()
}

fn find_localization<'a, P>(path: P, language: &'a str) -> impl Iterator<Item = PathBuf> + 'a
where P: AsRef<Path> {
    fs::read_dir(path.as_ref())
        .map_err(|error| warn!("{:?}, {}", path.as_ref(), error))
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .flat_map(move |path| -> Box<dyn Iterator<Item = PathBuf>> {
            if path.is_dir() {
                Box::new(find_localization(path, language))
            } else if
                path.extension() == Some(&FTL_EXT) &&
                path.file_stem()
                    .and_then(OsStr::to_str)
                    .map(|name| language.starts_with(name))
                    .unwrap_or(false)
            {
                // TODO: might want to make this smarter so it doesn't pick two matching languages
                // from the same directory. The problem can be avoided by name files smartly, but
                // still could make sense to improve
                Box::new(vec![path].into_iter())
            } else {
                Box::new(std::iter::empty())
            }
        })
}
