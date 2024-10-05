mod handlers;

use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Arc;
use tauri::AppHandle;
pub struct AppState {
    tauri: Arc<AppHandle>,
}

#[actix_web::main]
pub async fn init(app: Arc<AppHandle>) -> Result<(), io::Error> {
    let app_data: web::Data<AppState> = web::Data::new(AppState { tauri: app });

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(handlers::handle)
    })
    .bind(("0.0.0.0", 8729))
    .expect("failed to bind address")
    .run();

    println!("Server started at http://0.0.0.0:1111");

    http_server.await
}
