use std::ffi::OsStr;
use std::{fs, path::{Path, PathBuf}};
use lazy_static::lazy_static;
use log::warn;
use toml;
use serde::Deserialize;
use shared::*;
use crate::env;

const DESC_FILE: &'static str = "description.toml";
lazy_static! {
    static ref FTL_EXT: &'static OsStr = OsStr::new("ftl");
}

fn parse_toml<T, P>(path: P) -> Result<T, ()>
where
    for<'de> T: Deserialize<'de>,
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(path.as_ref()).map_err(|error| warn!("{:?}: {}", path.as_ref(), error))?;
    toml::from_str(&contents).map_err(|error| warn!("{:?}: {}", path.as_ref(), error))
}

pub fn load_directory<T, E, P, F>(path: P, loader: F) -> impl Iterator<Item = T>
where
    P: AsRef<Path>,
    F: FnMut(PathBuf) -> Result<T, E>,
{
    fs::read_dir(path.as_ref())
        .map_err(|error| warn!("{:?}, {}", path.as_ref(), error))
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .map(loader)
        .filter_map(Result::ok)
}

pub fn load_description<T, P: AsRef<Path>>(path: P) -> Result<T, ()>
where for<'de> T: Deserialize<'de> {
    parse_toml(path.as_ref().join(DESC_FILE))
}

pub fn load_universe(id: Id<Universe>) -> Result<Universe, ()> {
    let universe_directory = env::SCHEMA_DIR.join("universes").join(&id);
    let description: Description<Universe> = load_description(&universe_directory)?;
    Ok(Universe {
        description,
        stats: load_directory(universe_directory.join("stats"), load_description)
            .collect(),
        damage_types: load_directory(universe_directory.join("damage-types"), load_description)
            .collect(),
        resources: load_directory(universe_directory.join("resources"), load_description)
            .collect(),
        research: load_directory(universe_directory.join("research"), load_description)
            .collect(),
        unit_classes: load_directory(universe_directory.join("unit-classes"), load_description)
            // TODO: parse attributes and add to classes
            .collect(),
        units: load_directory(universe_directory.join("unit-types"), load_description)
            .collect(),
        modifier_classes: load_directory(universe_directory.join("modifier-classes"), load_description)
            // TODO: parse attributes and add to classes
            .collect(),
        modifiers: load_directory(universe_directory.join("modifier-types"), load_description)
            .collect(),
        tiles: load_directory(universe_directory.join("tile-types"), load_description)
            // TODO: parse attributes and add to classes
            .collect(),
        maps: load_directory(universe_directory.join("map-types"), load_description)
            .collect(),
        races: load_directory(universe_directory.join("races"), load_description)
            .collect(),
        attributes: Vec::default(),
    })
}

pub fn load_localization<S: AsRef<str>>(language: S) -> String {
    find_localization(&*env::SCHEMA_DIR, language.as_ref())
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
