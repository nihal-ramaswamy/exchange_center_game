#[get("/healthcheck")]
pub fn index() -> &'static str {
    "Hello, world!"
}
