use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

#[get("/hi")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
