use sqlx::PgPool;

mod startup;
mod routes;

#[rocket::main]
async fn main() -> std::io::Result<()> {
    let db_pool = PgPool::connect(std::env::var("DATABASE_URL").unwrap().as_str()).await.unwrap();
    let _ = startup::run(db_pool).await;

    Ok(())
}
