mod config;
mod errors;

use crate::errors::CustomError;
use axum::{extract::Extension, response::Html, routing::get, Router};
use deadpool_postgres::Pool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = config.create_pool();

    // build our application with a route
    let app = Router::new()
        .route("/", get(fortunes))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.unwrap();
}

async fn fortunes(Extension(pool): Extension<Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let fortunes = queries::fortunes::fortunes(&client).await?;

    let fortunes = format!("{:?}", fortunes);

    Ok(Html(fortunes))
}

// Include the generated source code
include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));


/*
#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = config.create_pool();

    let client = pool.get().await.unwrap();

    let fortunes = queries::fortunes::fortunes(&client).await.unwrap();

    dbg!(fortunes);
}

// Include the generated source code
include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));
*/