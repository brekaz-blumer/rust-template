extern crate rust_template;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use rust_template::events::reducer::Reducer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let reducer = Reducer {};
    actix_rt::spawn(async move { reducer.start_consumer().await });

    HttpServer::new(move || App::new())
        .bind("0.0.0.0:6000")?
        .run()
        .await
}
