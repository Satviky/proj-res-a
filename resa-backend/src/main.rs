use actix_web::{web, App, HttpServer, Responder};

mod db;

async fn greet() -> impl Responder {
    "Hello, Rescue Game!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::connect_to_mongodb().await.unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
