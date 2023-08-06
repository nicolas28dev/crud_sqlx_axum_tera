use controllers::authors_controller;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;
use tera::Tera;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use lazy_static::lazy_static;

mod models;
mod db_actions;
mod custom_helpers;
mod controllers;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
                }
            };
        tera
    };
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
        dotenvy::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        //let pool = PgPool::connect(&database_url).await.expect("Failed to create a connection pool");
        //PgPool::connect_lazy(&database_url).unwrap()

        let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("can't connect to database");

        let app = Router::new()
        .route("/author_form", get(authors_controller::show_author_form).post(authors_controller::accept_author_form))
        .route("/author/:guid", get(authors_controller::show_author).post(authors_controller::update_author_form))
        .route("/author/delete/:guid", post(authors_controller::delete_author)) //html cannot send delete request, so we use a post method
        .route("/", get(authors_controller::show_all_authors))
        .with_state(pool);

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}