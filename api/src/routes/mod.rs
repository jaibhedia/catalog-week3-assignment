use actix_web::{get, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
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
#[get("/api/pool-activity/{pool_id}")]
async fn pool_activity(
    path: web::Path<String>,
    query: web::Query<PoolActivityQuery>,
    service: web::Data<DepthService>,
) -> impl Responder {
    let pool_id = path.into_inner();
    let result = service
        .get_pool_activity(
            pool_id,
            query.start_date,
            query.end_date,
            query.page.unwrap_or(1),
            query.limit.unwrap_or(10),
        )
        .await;
    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(serde::Deserialize)]
struct PoolActivityQuery {
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    page: Option<i64>,
    limit: Option<i64>,
}
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/depth-history", web::get().to(get_depth_history))
            .route("/swaps-history", web::get().to(get_swaps_history))
            .route("/earnings-history", web::get().to(get_earnings_history))
            .route("/runepool-history", web::get().to(get_runepool_history))
    );
}