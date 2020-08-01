#[macro_use]
extern crate rocket;

use dotenv;
use env_logger;
use rocket::{response::content, State};
use std::env;

mod jwt;
mod schema;

use jwt::AuthenticatedAccount;
use schema::{Context, Database, Schema};

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket_async::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
async fn get_graphql_handler<'a>(
    database: State<'a, Database>,
    schema: State<'a, Schema>,
    account_id: Option<AuthenticatedAccount>,
    request: juniper_rocket_async::GraphQLRequest,
) -> juniper_rocket_async::GraphQLResponse {
    request
        .execute(
            &schema,
            &Context::new(database.clone(), account_id.map(Into::into)),
        )
        .await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql_handler<'a>(
    database: State<'a, Database>,
    schema: State<'a, Schema>,
    account_id: Option<AuthenticatedAccount>,
    request: juniper_rocket_async::GraphQLRequest,
) -> juniper_rocket_async::GraphQLResponse {
    request
        .execute(
            &schema,
            &Context::new(database.clone(), account_id.map(Into::into)),
        )
        .await
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let database_url = env::var("DATABASE_URL").unwrap();

    rocket::ignite()
        .manage(Database::connect(database_url).unwrap())
        .manage(schema::create())
        .mount(
            "/",
            routes![get_graphql_handler, post_graphql_handler, graphiql],
        )
        .launch()
        .await
        .unwrap()
}
