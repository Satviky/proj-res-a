mod db;
mod models;
// mod handlers;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = db::connect_to_mongodb().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            // .route("/messages", web::get().to(handlers::get_messages))
            // .route("/messages", web::post().to(handlers::add_message))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
