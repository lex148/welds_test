use crate::errors::Result;
use crate::models::dog::seed;
use crate::models::dog::Dog;
use crate::DbClient;
use actix_web::{get, HttpResponse};
use gumbo_lib::view::render;

#[get("/")]
async fn index(db: DbClient) -> Result<HttpResponse> {
    let start = chrono::Utc::now();
    let dogs = Dog::all().run(db.as_ref()).await?;
    let end = chrono::Utc::now();
    let diff: chrono::Duration = end - start;
    let micro = diff.num_microseconds().unwrap_or_default();
    let milli = micro as f64 / 1000.0;

    use crate::views::greetings::index::{View, ViewArgs};
    let args = ViewArgs::new(milli, dogs.len() as i32);
    render::<View, _, _>(args).await
}

#[get("/ab")]
async fn ab(db: DbClient) -> Result<HttpResponse> {
    let start = chrono::Utc::now();
    let _dogs = Dog::all().run(db.as_ref()).await?;
    let end = chrono::Utc::now();
    let diff: chrono::Duration = end - start;
    let micro = diff.num_microseconds().unwrap_or_default();
    let _milli = micro as f64 / 1000.0;
    Ok(HttpResponse::Ok().finish())
}

#[get("/reseed")]
async fn reseed(db: DbClient) -> Result<HttpResponse> {
    seed(&db).await?;
    Ok(HttpResponse::Ok().finish())
}
