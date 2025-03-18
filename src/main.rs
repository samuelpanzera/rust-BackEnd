use actix_web::{web, App, HttpServer, Responder};

async fn service_factory() -> impl Responder {
    "Hello world from service factory!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::scope("/").route("/teste", web::get().to(service_factory)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
