use actix_web::web;
use crate::routes::views;

pub fn routes(urls : &mut web::ServiceConfig) {
    urls.service(
        web::scope("/home")
            .service(views::home_views::index)
    );
}