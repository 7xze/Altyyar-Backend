use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::middleware::AuthenticatedUser;
use crate::models::cart::{AddToCartRequest, CartItem, UpdateCartRequest};
use crate::repository::cart;

pub async fn get_cart(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
) -> AppResult<Json<Vec<CartItem>>> {
    let items = cart::get_cart(&pool, &user.0.id).await?;
    Ok(Json(items))
}

pub async fn add_to_cart(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
    Json(req): Json<AddToCartRequest>,
) -> AppResult<Json<CartItem>> {
    let item = cart::add_to_cart(&pool, &user.0.id, req.service_id).await?;
    Ok(Json(item))
}

pub async fn update_cart_item(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
    Path(service_id): Path<Uuid>,
    Json(req): Json<UpdateCartRequest>,
) -> AppResult<Json<CartItem>> {
    let item = cart::update_cart_quantity(&pool, &user.0.id, service_id, req.quantity).await?;
    Ok(Json(item))
}

pub async fn remove_from_cart(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
    Path(service_id): Path<Uuid>,
) -> AppResult<Json<()>> {
    cart::remove_from_cart(&pool, &user.0.id, service_id).await?;
    Ok(Json(()))
}

pub async fn clear_cart(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
) -> AppResult<Json<()>> {
    cart::clear_cart(&pool, &user.0.id).await?;
    Ok(Json(()))
}
