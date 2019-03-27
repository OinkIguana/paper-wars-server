use std::{fs, path::Path};
use shared::{Id, Universe, Description};
use warp::{path, reject::not_found, Filter};
use dotenv;

mod env;
mod schema;
mod filters;

fn main() {
    dotenv::dotenv().ok();

    let universes_path = Path::new(&*env::SCHEMA_DIR).join("universes");
    let list_all_universes = path::end()
        .map(move || fs::read_dir(&universes_path)
            .expect("The schema directory must exist and be readable")
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .filter_map(|path| schema::load_description(&path).ok())
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
