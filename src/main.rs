use actix_web::{HttpServer, web, HttpResponse, get, App};

#[get("/")]
async fn index() -> HttpResponse {
    return HttpResponse::Ok().body("soon");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/")
                .service(index)
        )
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}