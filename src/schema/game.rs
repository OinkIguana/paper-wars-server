use warp::{Rejection, reject::not_found};
use shared::*;
use shared::collections::KeyedMap;
use crate::env;
use super::{parse_ron, write_ron, load_universe};

pub fn new_game(new_game: api::NewGame) -> Result<Game, Rejection> {
    let universe = load_universe(&new_game.universe)?;
    let game = Game {
        id: Id::new(),
        name: new_game.name,
        universe: new_game.universe,
        players: KeyedMap::default(),
        map: universe.maps.get(new_game.map)
            .ok_or_else(not_found)?
            .clone()
            .into(),
        units: vec![],
    };
    let game_file = env::SCHEMA_DIR.join("games").join(&game.id).join("game.ron");
    write_ron(game_file, &game)?;
    Ok(game)
}

pub fn load_game(id: Id<Game>) -> Result<Game, Rejection> {
    let game_file = env::SCHEMA_DIR.join("games").join(&id).join("game.ron");
    parse_ron(game_file)
}
