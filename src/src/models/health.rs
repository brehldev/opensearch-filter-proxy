use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
}
