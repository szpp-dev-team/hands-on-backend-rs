use actix_web::{error::ErrorUnauthorized, http::header, FromRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub id: i32,
}

pub const SECRET: &str = "seeecccccreeeeeeeeeeeeeeett";

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Claims, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let auth = req.headers().get(header::AUTHORIZATION);
        match auth {
            Some(auth) => {
                let token = auth
                    .to_str()
                    .unwrap()
                    .split("Bearer")
                    .collect::<Vec<_>>()
                    .get(1)
                    .unwrap()
                    .trim();
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(SECRET.as_bytes()),
                    &Validation::default(),
                ) {
                    Ok(c) => ok(c.claims),
                    Err(_e) => err(ErrorUnauthorized("invalid jwt token")),
                }
            }
            None => err(ErrorUnauthorized("blocked")),
        }
    }
}
