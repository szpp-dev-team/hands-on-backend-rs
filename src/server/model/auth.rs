use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SigninPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SigninResponse {
    pub token: String,
}

impl SigninResponse {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
