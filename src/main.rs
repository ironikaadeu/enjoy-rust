use actix_web::{get, App, HttpServer, Responder};


#[get("/")]
async fn index() -> impl Responder {
    "Hello world!!!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind(("web", 8080))?
    .run()
    .await
}
