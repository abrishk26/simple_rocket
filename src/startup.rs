use std::future::Future;
use sqlx::PgPool;
use crate::routes::health_check::*;

pub fn run(db_pool: PgPool) -> impl Future {
    let server = rocket::build()
        .mount("/", rocket::routes![health_check])
        .manage(db_pool.clone())
        .launch();

    return server;
}