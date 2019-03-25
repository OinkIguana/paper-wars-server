use log::error;
use warp::{path, Filter, Future, Stream, ws::Ws2};

fn main() {
    let game = path!("game" / String)
        .and(warp::ws2())
        .map(|game_id: String, ws: Ws2| {
            ws.on_upgrade(move |ws| {
                let (tx, rx) = ws.split();
                rx.forward(tx).map(|_| ()).map_err(|e| {
                    error!("Error: {}", e);
                })
            })
        });
    let load = path!("load" / String)
        .map(|game_id: String| {
            "Hello"
        });
    let routes = warp::get2().and(load).or(game);
    warp::serve(routes).run(([0, 0, 0, 0], 15273));
}
