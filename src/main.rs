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
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    rocket::ignite()
        .manage(Context::new(env::var("DATABASE_URL").unwrap()).unwrap())
        .manage(schema::create())
        .mount(
            "/",
            routes![get_graphql_handler, post_graphql_handler, graphiql],
        )
        .launch();
}
