use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::middleware::AuthenticatedUser;
use crate::models::service::{
    CreateServiceRequest, MarketplaceService, UpdateServiceRequest,
};
use crate::repository::services;

#[derive(Deserialize)]
pub struct ListServicesParams {
    category: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchServicesParams {
    q: String,
}

pub async fn list_services(
    State(pool): State<PgPool>,
    Query(params): Query<ListServicesParams>,
) -> AppResult<Json<Vec<MarketplaceService>>> {
    let services = services::list_services(&pool, params.category.as_deref()).await?;
    Ok(Json(services))
}

pub async fn search_services(
    State(pool): State<PgPool>,
    Query(params): Query<SearchServicesParams>,
) -> AppResult<Json<Vec<MarketplaceService>>> {
    let services = services::search_services(&pool, &params.q).await?;
    Ok(Json(services))
}

pub async fn get_service(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<MarketplaceService>> {
    let service = services::get_service(&pool, id).await?;
    Ok(Json(service))
}

pub async fn create_service(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
    Json(req): Json<CreateServiceRequest>,
) -> AppResult<Json<MarketplaceService>> {
    let price_cents = (req.price * 100.0).round() as i64;

    let service = services::create_service(
        &pool,
        &user.0.id,
        &user.0.display_name,
        &req.title,
        &req.description,
        price_cents,
        &req.images,
        &req.category.to_uppercase(),
    )
    .await?;

    Ok(Json(service))
}

pub async fn update_service(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateServiceRequest>,
) -> AppResult<Json<MarketplaceService>> {
    let price_cents = req.price.map(|p| (p * 100.0).round() as i64);

    let service = services::update_service(
        &pool,
        id,
        &user.0.id,
        req.title.as_deref(),
        req.description.as_deref(),
        price_cents,
        req.images.as_deref(),
        req.category.as_deref(),
    )
    .await?;

    Ok(Json(service))
}

pub async fn delete_service(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<()>> {
    services::delete_service(&pool, id, &user.0.id).await?;
    Ok(Json(()))
}

pub async fn get_seller_services(
    State(pool): State<PgPool>,
    Path(seller_id): Path<String>,
) -> AppResult<Json<Vec<MarketplaceService>>> {
    let services = services::get_services_by_seller(&pool, &seller_id).await?;
    Ok(Json(services))
}

pub async fn get_my_services(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
) -> AppResult<Json<Vec<MarketplaceService>>> {
    let services = services::get_services_by_seller(&pool, &user.0.id).await?;
    Ok(Json(services))
}
