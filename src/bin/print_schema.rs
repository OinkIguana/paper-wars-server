use lib::schema::{self, Context, Database};
use std::env;

fn main() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();
    let database = Database::connect(database_url).unwrap();
    let context = Context::new(database, None);
    let schema = schema::create();
    let output = juniper::introspect(&schema, &context, Default::default()).unwrap();
    println!("{}", output.0);
}
