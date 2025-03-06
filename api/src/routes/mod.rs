use actix_web::{get, web, HttpResponse};
use crate::models::QueryParams;
use crate::services::DepthService;

pub async fn get_depth_history(query: web::Query<QueryParams>, service: web::Data<DepthService>) -> HttpResponse {
    match service.get_depths(&query).await {
        Ok(depths) => HttpResponse::Ok().json(depths),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_swaps_history(query: web::Query<QueryParams>, service: web::Data<DepthService>) -> HttpResponse {
    match service.get_swaps(&query).await {
        Ok(swaps) => HttpResponse::Ok().json(swaps),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_earnings_history(query: web::Query<QueryParams>, service: web::Data<DepthService>) -> HttpResponse {
    match service.get_earnings(&query).await {
        Ok(earnings) => HttpResponse::Ok().json(earnings),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_runepool_history(query: web::Query<QueryParams>, service: web::Data<DepthService>) -> HttpResponse {
    match service.get_runepools(&query).await {
        Ok(runepools) => HttpResponse::Ok().json(runepools),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/pool-activity/{pool_id}")]
pub async fn get_pool_activity(
    path: web::Path<String>,
    query: web::Query<QueryParams>,
    service: web::Data<DepthService>,
) -> HttpResponse {
    match service.get_pool_activity(path.into_inner(), &query).await {
        Ok(activity) => HttpResponse::Ok().json(activity),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/depth-history", web::get().to(get_depth_history))
            .route("/swaps-history", web::get().to(get_swaps_history))
            .route("/earnings-history", web::get().to(get_earnings_history))
            .route("/runepool-history", web::get().to(get_runepool_history))
            .route("/pool-activity/{pool_id}", web::get().to(get_pool_activity))
    );
}