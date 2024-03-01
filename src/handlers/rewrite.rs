use std::sync::Arc;
use actix_web::{get, HttpResponse, Responder};
use actix_web::web::{Data, Path, ServiceConfig};
use crate::handlers::links::{create_link, delete_link, find_link, list, update_link};
use crate::repository::mongodb_link::MongoRepo;


#[get("/{hash}")]
async fn goto(
    link_repository: Data<Arc<MongoRepo>>,
    path: Path<String>,
) -> impl Responder {
    let hash = path.into_inner();
    println!("{:?}", hash);
    HttpResponse::Ok().json(hash)
}


pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(goto);
}