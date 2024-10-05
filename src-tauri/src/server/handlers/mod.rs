use actix_web::{get, web, Responder};
use tauri::{Emitter, Manager};

use crate::server::AppState;
use serde::Serialize;

#[derive(Serialize)]
struct TestT {
    a: String,
}
// #[post("/api/test")]
#[get("/api/test")]
pub async fn handle(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let tauri = data.tauri.get_webview_window("main").unwrap();
    let _ = tauri.emit("test", "test_string");
    let text = "hello world";
    println!("{}", text);

    let obj = TestT {
        a: "Hello, world!".to_string(),
    };
    Ok(web::Json(obj))
}
