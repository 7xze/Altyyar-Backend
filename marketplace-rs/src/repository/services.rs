use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::service::MarketplaceService;

pub async fn list_services(
    pool: &PgPool,
    category: Option<&str>,
) -> AppResult<Vec<MarketplaceService>> {
    let services = if let Some(cat) = category {
        sqlx::query_as::<_, MarketplaceService>(
            "SELECT * FROM marketplace_services WHERE is_active = TRUE AND category = $1 ORDER BY created_at DESC",
        )
        .bind(cat)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, MarketplaceService>(
            "SELECT * FROM marketplace_services WHERE is_active = TRUE ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await?
    };

    Ok(services)
}

pub async fn search_services(pool: &PgPool, query: &str) -> AppResult<Vec<MarketplaceService>> {
    let pattern = format!("%{}%", query);
    let services = sqlx::query_as::<_, MarketplaceService>(
        "SELECT * FROM marketplace_services
         WHERE is_active = TRUE
         AND (title ILIKE $1 OR description ILIKE $1)
         ORDER BY created_at DESC",
    )
    .bind(&pattern)
    .fetch_all(pool)
    .await?;

    Ok(services)
}

pub async fn get_service(pool: &PgPool, id: Uuid) -> AppResult<MarketplaceService> {
    sqlx::query_as::<_, MarketplaceService>(
        "SELECT * FROM marketplace_services WHERE id = $1 AND is_active = TRUE",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Service not found".into()))
}

pub async fn create_service(
    pool: &PgPool,
    seller_id: &str,
    seller_name: &str,
    title: &str,
    description: &str,
    price_cents: i64,
    images: &[String],
    category: &str,
) -> AppResult<MarketplaceService> {
    let valid_categories = ["DESIGN", "PHOTOGRAPHY", "EDITING"];
    if !valid_categories.contains(&category) {
        return Err(AppError::Validation(format!(
            "Invalid category '{}'. Must be one of: DESIGN, PHOTOGRAPHY, EDITING",
            category
        )));
    }

    if title.trim().is_empty() {
        return Err(AppError::Validation("Title is required".into()));
    }

    if price_cents < 0 {
        return Err(AppError::Validation("Price cannot be negative".into()));
    }

    let service = sqlx::query_as::<_, MarketplaceService>(
        "INSERT INTO marketplace_services (title, description, price, images, seller_id, seller_name, category)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *",
    )
    .bind(title.trim())
    .bind(description.trim())
    .bind(price_cents)
    .bind(images)
    .bind(seller_id)
    .bind(seller_name)
    .bind(category)
    .fetch_one(pool)
    .await?;

    Ok(service)
}

pub async fn update_service(
    pool: &PgPool,
    id: Uuid,
    seller_id: &str,
    title: Option<&str>,
    description: Option<&str>,
    price_cents: Option<i64>,
    images: Option<&[String]>,
    category: Option<&str>,
) -> AppResult<MarketplaceService> {
    let existing = get_service(pool, id).await?;

    if existing.seller_id != seller_id {
        return Err(AppError::Unauthorized("You can only edit your own services".into()));
    }

    let title = title.unwrap_or(&existing.title);
    let description = description.unwrap_or(&existing.description);
    let price_cents = price_cents.unwrap_or(existing.price);
    let images = images.unwrap_or(&existing.images);
    let category = category.unwrap_or(&existing.category);

    if let Some(cat) = category {
        let valid_categories = ["DESIGN", "PHOTOGRAPHY", "EDITING"];
        if !valid_categories.contains(&cat) {
            return Err(AppError::Validation(format!(
                "Invalid category '{}'", cat
            )));
        }
    }

    let service = sqlx::query_as::<_, MarketplaceService>(
        "UPDATE marketplace_services
         SET title = $1, description = $2, price = $3, images = $4, category = $5, updated_at = NOW()
         WHERE id = $6
         RETURNING *",
    )
    .bind(title)
    .bind(description)
    .bind(price_cents)
    .bind(images)
    .bind(category)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(service)
}

pub async fn delete_service(pool: &PgPool, id: Uuid, seller_id: &str) -> AppResult<()> {
    let existing = get_service(pool, id).await?;

    if existing.seller_id != seller_id {
        return Err(AppError::Unauthorized("You can only delete your own services".into()));
    }

    sqlx::query("UPDATE marketplace_services SET is_active = FALSE, updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_services_by_seller(pool: &PgPool, seller_id: &str) -> AppResult<Vec<MarketplaceService>> {
    let services = sqlx::query_as::<_, MarketplaceService>(
        "SELECT * FROM marketplace_services WHERE seller_id = $1 ORDER BY created_at DESC",
    )
    .bind(seller_id)
    .fetch_all(pool)
    .await?;

    Ok(services)
}
