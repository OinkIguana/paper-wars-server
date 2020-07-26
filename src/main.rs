#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use dotenv;
use env_logger;
use rocket::{response::content, State};
use std::env;

mod schema;

use schema::{Context, Schema};

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket_async::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
async fn get_graphql_handler<'a>(
    context: State<'a, Context>,
    schema: State<'a, Schema>,
    request: juniper_rocket_async::GraphQLRequest,
) -> juniper_rocket_async::GraphQLResponse {
    request.execute(&schema, &context).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql_handler<'a>(
    context: State<'a, Context>,
    schema: State<'a, Schema>,
    request: juniper_rocket_async::GraphQLRequest,
) -> juniper_rocket_async::GraphQLResponse {
    request.execute(&schema, &context).await
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    rocket::ignite()
        .manage(Context::new(env::var("DATABASE_URL").unwrap()).unwrap())
        .manage(schema::create())
        .mount(
            "/",
            routes![get_graphql_handler, post_graphql_handler, graphiql],
        )
        .launch()
        .await
        .unwrap()
}
