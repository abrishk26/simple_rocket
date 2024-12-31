use rocket::http::Status;

#[rocket::get("/")]
pub async fn health_check() -> Status {
    Status::Ok
}