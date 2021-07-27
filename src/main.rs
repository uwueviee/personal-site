mod routes;

use actix_web::{HttpServer, web, HttpResponse, get, App};
use std::env;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;

#[get("/")]
async fn index() -> HttpResponse {
    return HttpResponse::Ok().body("soon");
}

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    dotenv::dotenv().ok();

    // Load environment variables
    let listen_address = env::var("LISTEN_ADDRESS")
        .expect("LISTEN_ADDRESS not defined");
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not defined");

    // Create database pool
    let connman = ConnectionManager::<MysqlConnection>::new(database_url);
    let dbpool = r2d2::Pool::builder()
        .build(connman)
        .expect("Failed to create database pool");

    println!("Listening on {}", &listen_address);

    // Create HttpServer
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(dbpool.clone()))
            .service(
            web::scope("/")
                .service(index)
        )
    })
        .bind(listen_address)?
        .run()
        .await
}