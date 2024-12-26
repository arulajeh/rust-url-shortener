use sqlx::{PgPool, Result};
use crate::models::database::url::Url;

/// Insert a new short URL into the database
pub async fn insert_url(pool: &PgPool, shorten: &str, url: &str) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO urls (shorten, url, created_at, counter)
        VALUES ($1, $2, NOW(), 0)
        "#,
        shorten,
        url
    )
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a URL by its short key
pub async fn get_url_by_shorten(pool: &PgPool, shorten: &str) -> Result<Option<Url>> {
    let result = sqlx::query_as::<_, Url>(
        r#"
        SELECT id, shorten, url, created_at, counter
        FROM urls
        WHERE shorten = $1
        "#,
    )
        .bind(shorten) // Bind parameter
        .fetch_optional(pool)
        .await?;

    Ok(result)
}
