mod user;
mod repository;

use std::{sync::{Arc, atomic::{AtomicU16, Ordering}}};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use repository::{MemoryRepository, Repository};
use uuid::Uuid;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("Mundo");
    format!("Hola {}!", &name)
}

async fn get_user(user_id: web::Path<Uuid>) -> HttpResponse {

    let repo = MemoryRepository::default();

    match repo.get_user(&user_id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("Not found en repositorio")
    }
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("127.0.0.1:{}", port);

    println!("Empezando nuestro servicio");
    let thread_counter = Arc::new(AtomicU16::new(1));
    HttpServer::new(move || {

        println!("Empezando el hilo {}", thread_counter.fetch_add(1, Ordering::SeqCst));
        let thread_index = thread_counter.load(Ordering::SeqCst);

        App::new()
            .service(web::resource("/user/{user_id}").route(web::get().to(get_user)))
            .route("/", web::get().to(greet))
            .route(
                "/health", 
                web::get().to(move || {
                    HttpResponse::Ok()
                        .header("thread-id", thread_index.to_string())
                        .finish()
                }),
            )
            .route("/str", web::get().to(|| async {"Hola Rust"}))
            .route("/{name}", web::get().to(greet))
    })
    .bind(&address)?
    .run()
    .await
}