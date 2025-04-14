use actix_web::{App, HttpServer, middleware::Logger, web};
use env_logger::Env;
use log::info;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("🚀 Server is starting at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .default_service(web::route().to(routes::not_found_routes::not_found))
            .service(
                web::scope("/bff_v001")
                    .configure(routes::home_routes::routes)
                   
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
