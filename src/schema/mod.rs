use log::warn;
use std::{fs, path::{Path, PathBuf}};
use toml;
use serde::Deserialize;
use shared::*;
use shared::collections::KeyedMap;
use crate::env;

const DESC_FILE: &'static str = "description.toml";

fn parse_toml<T, P>(path: P) -> Result<T, ()> 
where
    for<'de> T: Deserialize<'de>,
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(path).map_err(|_| ())?;
    toml::from_str(&contents).map_err(|error| warn!("{}", error))
}

pub fn load_directory<T, E, P, F>(path: P, loader: F) -> impl Iterator<Item = T>
where
    P: AsRef<Path>,
    F: FnMut(PathBuf) -> Result<T, E>,
{
    fs::read_dir(path.as_ref())
        .expect("The directory must exist and be readable")
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
            .collect(),
        units: load_directory(universe_directory.join("unit-types"), load_description)
            .collect(),
        modifier_classes: KeyedMap::default(),
        modifiers: KeyedMap::default(),
        tiles: KeyedMap::default(),
        maps: KeyedMap::default(),
        races: KeyedMap::default(),
        attributes: Vec::default(),
    })
}
