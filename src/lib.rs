use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("localhost:8000")?
        .run();

    Ok(server)
}
