use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::middleware::AuthenticatedUser;
use crate::models::order::{CreateOrderRequest, OrderWithItems};
use crate::repository::orders;

pub async fn create_order(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
    Json(req): Json<CreateOrderRequest>,
) -> AppResult<Json<OrderWithItems>> {
    let notes = req.notes.unwrap_or_default();
    let order = orders::create_order(&pool, &user.0.id, &notes).await?;
    Ok(Json(order))
}

pub async fn get_my_orders(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
) -> AppResult<Json<Vec<OrderWithItems>>> {
    let orders = orders::get_orders_by_buyer(&pool, &user.0.id).await?;
    Ok(Json(orders))
}

pub async fn get_order(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
    Path(order_id): Path<Uuid>,
) -> AppResult<Json<OrderWithItems>> {
    let order = orders::get_order_by_id(&pool, order_id).await?;

    if order.buyer_id != user.0.id {
        let seller_orders = orders::get_orders_by_seller(&pool, &user.0.id).await?;
        if !seller_orders.iter().any(|o| o.id == order_id) {
            return Err(crate::error::AppError::Unauthorized("Not your order".into()));
        }
    }

    Ok(Json(order))
}
