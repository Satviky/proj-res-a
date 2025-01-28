use actix_web::{web, HttpResponse};
use mongodb::bson::doc;
use mongodb::Client;
use serde_json::json;

use crate::models::Message;

pub async fn get_messages(client: web::Data<Client>) -> HttpResponse {
    let collection = client.database("rescue").collection::<Message>("messages");
    let cursor = collection.find(None, None).await.unwrap();
    let messages: Vec<Message> = cursor.try_collect().await.unwrap();

    HttpResponse::Ok().json(messages)
}

pub async fn add_message(client: web::Data<Client>, msg: web::Json<Message>) -> HttpResponse {
    let collection = client.database("rescue").collection("messages");
    let new_message = Message {
        sender: msg.sender.clone(),
        content: msg.content.clone(),
        timestamp: msg.timestamp.clone(),
    };
    collection.insert_one(new_message, None).await.unwrap();

    HttpResponse::Created().json(json!({"status": "Message added"}))
}
