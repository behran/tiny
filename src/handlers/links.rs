use std::sync::Arc;
use actix_web::{delete, get, HttpResponse, patch, post, Responder, web::{Data, ServiceConfig}};
use actix_web::web::{Json, Path};
use bson::oid::ObjectId;
use crate::models::link::Link;
use crate::repository::mongodb_link::MongoRepo;

#[get("")]
async fn list(
    link_repository: Data<Arc<MongoRepo>>,
) -> impl Responder {
    let links = link_repository.get_all_links().await;
    match links {
        Ok(links) => HttpResponse::Ok().json(links),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[post("")]
async fn create_link(
    link_repository: Data<Arc<MongoRepo>>,
    payload: Json<Link>,
) -> impl Responder {
    let data = Link {
        id: None,
        link: payload.link.to_owned(),
    };
    let result = link_repository.create_link(data).await;

    match result {
        Ok(link) => HttpResponse::Ok().json(link),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/{id}")]
async fn delete_link(
    link_repository: Data<Arc<MongoRepo>>,
    path: Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let result = link_repository.delete_link(id).await;

    match result {
        Ok(delete_result) => HttpResponse::Ok().json(delete_result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{id}")]
async fn find_link(
    link_repository: Data<Arc<MongoRepo>>,
    path: Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let result = link_repository.find_link(id).await;

    match result {
        Ok(link) => HttpResponse::Ok().json(link),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[patch("/{id}")]
async fn update_link(
    link_repository: Data<Arc<MongoRepo>>,
    path: Path<String>,
    payload: Json<Link>,
) -> impl Responder {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let link = Link {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        link: payload.link.to_owned(),
    };

    let result = link_repository.update_link(id, link).await;
    match result {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


pub fn config(cfg: &mut ServiceConfig) {
    cfg.
        service(list).
        service(find_link).
        service(delete_link).
        service(update_link).
        service(create_link);
}


