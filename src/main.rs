mod config;
mod repository;
mod handlers;
mod models;
mod tools;

use dotenv::dotenv;

use std::sync::Arc;
use actix_web::{web::Data, App, HttpServer, Responder, web};
use actix_web_prom::PrometheusMetricsBuilder;

use config::{Config};
use crate::handlers::{links, rewrite, metrics};
use crate::repository::mongodb_link::MongoRepo;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = Config::new();

    println!("{:?}", config);

    let prometheus = PrometheusMetricsBuilder::new("api")
        .build()
        .unwrap();

    let prometheus_data = Data::new(prometheus.clone());

    let mongodb = MongoRepo::new(config.clone()).await;
    let db_data = Data::new(Arc::new(mongodb));
    HttpServer::new(move ||
        App::new()
            .wrap(prometheus.clone())
            .service(
                web::scope("/short-links")
                    .configure(links::config))
            .service(
                web::scope("/rewrite")
                    .configure(rewrite::config))
            .configure(metrics::config)
            .app_data(db_data.clone())
            .app_data(prometheus_data.clone())
    )
        .bind((config.app.hostname, config.app.port))?
        .run()
        .await
}




