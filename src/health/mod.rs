#[get("/health")]
pub fn health() -> String {
    String::from("healthy")
}
