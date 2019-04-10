use shared::*;
use warp::{path, Filter, Rejection};
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
        .and_then(move || -> Result<Vec<_>, Rejection> {
            Ok(
                schema::load_directory(&universes_path, schema::load_description)?
                    .collect::<Vec<Description<Universe>>>()
            )
        })
        .map(|universes| filters::cbor(&universes));

    let load_universe = path::param::<Id<Universe>>()
        .and(path::end())
        .and_then(|id| schema::load_universe(&id))
        .map(|universe| filters::cbor(&universe));

    let localize = path!("l10n" / "universes")
        .and(warp::filters::path::param::<Id<Universe>>())
        .and(warp::filters::header::header("Accept-Language"))
        .map(|id, language: String| (id, accept_language::parse(&language)))
        .map(|(id, languages): (Id<Universe>, Vec<String>)| schema::load_localization(id, &languages[0]))
        .map(|ftl: String| filters::cbor(&ftl));

    let universes = path!("universe")
        .and(
            load_universe
            .or(list_all_universes)
        );

    let new_game = path!("new")
        .and(warp::filters::body::content_length_limit(512))
        .and(warp::filters::body::concat())
        .and_then(filters::from_cbor)
        .map(|new_game: api::NewGame| schema::new_game(new_game))
        .map(|game| filters::cbor(&game));

    let load_game = path::param::<Id<Game>>()
        .and(path::end())
        .and_then(schema::load_game)
        .map(|game| filters::cbor(&game));

    let games = warp::get2().and(path("game").and(load_game))
        .or(warp::post2().and(path("game").and(new_game)));

    let maker = path("maker").and(warp::fs::dir(&*env::MAKER_DIR));
    let player = path("player").and(warp::fs::dir(&*env::PLAYER_DIR));

    let routes = warp::get2()
        .and(
            localize
            .or(universes)
            .or(maker)
            .or(player)
        )
        .or(games)
        .or_else(|_| Err(warp::reject::not_found()))
        .with(warp::filters::log::log("server"));

    println!();
    println!("Server is listening on port {}", *env::PORT);
    println!();

    warp::serve(routes).run(([0, 0, 0, 0], *env::PORT));
}
