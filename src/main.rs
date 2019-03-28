use shared::{Id, Universe, Description};
use warp::{path, reject::not_found, Filter};
use dotenv;
use env_logger;

mod env;
mod schema;
mod filters;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let universes_path = env::SCHEMA_DIR.join("universes");
    let list_all_universes = path::end()
        .map(move || schema::load_directory(&universes_path, |path| schema::load_description(path))
            .collect::<Vec<Description<Universe>>>()
        )
        .map(|universes| filters::cbor(&universes));

    let load_universe = path::param::<Id<Universe>>()
        .map(schema::load_universe)
        .and_then(|universe: Result<Universe, ()>| universe
            .as_ref()
            .map(filters::cbor)
            .map_err(|_| not_found())
        );

    let universe = path!("universe").and(load_universe.or(list_all_universes));

    let routes = warp::get2().and(universe);

    println!();
    println!("Server is listening on port {}", *env::PORT);
    println!();

    warp::serve(routes).run(([0, 0, 0, 0], *env::PORT));
}
