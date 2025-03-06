use actix_web::{web, HttpResponse};
use crate::models::QueryParams;
use crate::services::DataService;

pub async fn get_depths(
    query: web::Query<QueryParams>,
    data_service: web::Data<DataService>,
) -> HttpResponse {
    match data_service.get_depths(&query).await {
        Ok(depths) => HttpResponse::Ok().json(depths),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_swaps(
    query: web::Query<QueryParams>,
    data_service: web::Data<DataService>,
) -> HttpResponse {
    match data_service.get_swaps(&query).await {
        Ok(swaps) => HttpResponse::Ok().json(swaps),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_earnings(
    query: web::Query<QueryParams>,
    data_service: web::Data<DataService>,
) -> HttpResponse {
    match data_service.get_earnings(&query).await {
        Ok(earnings) => HttpResponse::Ok().json(earnings),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_runepool(
    query: web::Query<QueryParams>,
    data_service: web::Data<DataService>,
) -> HttpResponse {
    match data_service.get_runepool(&query).await {
        Ok(runepool) => HttpResponse::Ok().json(runepool),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}