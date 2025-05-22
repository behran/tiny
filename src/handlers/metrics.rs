use actix_web::{get, HttpResponse, Responder};
use actix_web::web::{Data, ServiceConfig};
use actix_web_prom::PrometheusMetrics;

#[get("/metrics")]
async fn metrics(prometheus: Data<PrometheusMetrics>) -> impl Responder {
    let body = prometheus.render();
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(body)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(metrics);
}
