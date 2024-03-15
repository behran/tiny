use std::sync::Arc;
use actix_web::{get, HttpResponse, Responder};
use actix_web::web::{Data, Path, Redirect, ServiceConfig};
use crate::repository::mongodb_link::MongoRepo;


#[get("/{hash}")]
async fn goto(
    link_repository: Data<Arc<MongoRepo>>,
    path: Path<String>,
) -> impl Responder {
    let hash = path.into_inner();
    let result = link_repository.find_link_by_hash(hash).await;
    match result {
        Ok(link) => HttpResponse::Found().append_header(("Location", link.link)).finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(goto);
}