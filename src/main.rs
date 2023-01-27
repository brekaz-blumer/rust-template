extern crate rust_template;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use rust_template::events::reducer::Reducer;
use rust_template::graphql::{configure_service, create_schema_with_context};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let reducer = Reducer {};
    actix_rt::spawn(async move { reducer.start_consumer().await });
    let schema = web::Data::new(create_schema_with_context());
    println!("GraphiQL IDE: http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .configure(configure_service)
            .app_data(schema.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
