use sqlx::PgPool;
use uuid::Uuid;

use crate::models::author::Author;
use crate::models::author_form::AuthorForm;

pub async fn insert_author(pool: &PgPool, author: &AuthorForm) -> Result<Author, sqlx::Error> {
    sqlx::query_as::<_, Author>(
        r#"
        INSERT INTO public."Author" ("id", "first_name", "last_name")
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(author.first_name.clone())
    .bind(author.last_name.clone())
    .fetch_one(pool).await
} 

pub async fn update_author(pool: &PgPool, author_id: &Uuid, author: &AuthorForm) -> Result<Author, sqlx::Error> {
    sqlx::query_as::<_, Author>(
        r#"
        UPDATE public."Author"
        SET "first_name" = $1, "last_name" = $2
        WHERE "id" = $3
        RETURNING *
        "#,
    )
    .bind(author.first_name.clone())
    .bind(author.last_name.clone())
    .bind(author_id)
    .fetch_one(pool).await
} 

pub async fn select_author(pool: &PgPool, author_id: &Uuid) -> Result<Author, sqlx::Error> {
    sqlx::query_as::<_, Author>(
        r#"
        SELECt * FROM public."Author" 
        WHERE "id" = $1
        "#,
    )
    .bind(author_id)
    .fetch_one(pool).await
}

pub async fn select_all_authors(pool: &PgPool) -> Result<Vec<Author>, sqlx::Error> {
    sqlx::query_as::<_, Author>(
        r#"
        SELECT * FROM public."Author"
        "#,
    )
    .fetch_all(pool).await
}

pub async fn delete_author(pool: &PgPool, author_id: &Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM public."Author"
        WHERE "id" = $1
        "#,
    )
    .bind(author_id)
    .execute(pool).await?;
    
    Ok(result.rows_affected())
}