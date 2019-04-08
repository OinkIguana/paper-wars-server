use std::path::Path;
use serde::Deserialize;
use warp::Rejection;
use shared::*;
use crate::env;
use super::{parse_toml, load_directory};

const DESC_FILE: &'static str = "description.toml";

pub fn load_description<T, P: AsRef<Path>>(path: P) -> Result<T, Rejection>
where for<'de> T: Deserialize<'de> {
    parse_toml(path.as_ref().join(DESC_FILE))
}

pub fn load_universe(id: &Id<Universe>) -> Result<Universe, Rejection> {
    let universe_directory = env::SCHEMA_DIR.join("universes").join(id);
    let description: Description<Universe> = load_description(&universe_directory)?;
    Ok(Universe {
        description,
        stats: load_directory(universe_directory.join("stats"), load_description)?
            .collect(),
        damage_types: load_directory(universe_directory.join("damage-types"), load_description)?
            .collect(),
        resources: load_directory(universe_directory.join("resources"), load_description)?
            .collect(),
        research: load_directory(universe_directory.join("research"), load_description)?
            .collect(),
        unit_classes: load_directory(universe_directory.join("unit-classes"), load_description)?
            // TODO: parse attributes and add to classes
            .collect(),
        units: load_directory(universe_directory.join("unit-types"), load_description)?
            .collect(),
        modifier_classes: load_directory(universe_directory.join("modifier-classes"), load_description)?
            // TODO: parse attributes and add to classes
            .collect(),
        modifiers: load_directory(universe_directory.join("modifier-types"), load_description)?
            .collect(),
        tiles: load_directory(universe_directory.join("tile-types"), load_description)?
            // TODO: parse attributes and add to classes
            .collect(),
        maps: load_directory(universe_directory.join("map-types"), load_description)?
            .collect(),
        races: load_directory(universe_directory.join("races"), load_description)?
            .collect(),
        attributes: Vec::default(),
    })
}
