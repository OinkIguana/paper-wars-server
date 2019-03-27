use std::{fs, path::Path};
use toml;
use shared::*;
use shared::collections::KeyedMap;
use crate::env;

pub fn load_description<T, P: AsRef<Path>>(path: P) -> Result<Description<T>, ()> {
    let description_path = path.as_ref().join("description.toml");
    let contents = fs::read_to_string(&description_path).map_err(|_| ())?;
    toml::from_str(&contents).map_err(|_| ())
}

pub fn load_universe(id: Id<Universe>) -> Result<Universe, ()> {
    let universe_directory = Path::new(&*env::SCHEMA_DIR).join("universes").join(&id);
    let description: Description<Universe> = load_description(universe_directory)?;
    Ok(Universe {
        id,
        name: description.name,
        description: description.description,
        stats: KeyedMap::default(),
        damage_types: KeyedMap::default(),
        resources: KeyedMap::default(),
        research: KeyedMap::default(),
        unit_classes: KeyedMap::default(),
        units: KeyedMap::default(),
        modifier_class: KeyedMap::default(),
        modifiers: KeyedMap::default(),
        tiles: KeyedMap::default(),
        maps: KeyedMap::default(),
        races: KeyedMap::default(),
        attributes: Vec::default(),
    })
}
