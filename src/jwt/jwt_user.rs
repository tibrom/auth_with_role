use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use super::claims::Claims;
use crate::conf::credentials::CredentialsManager;

pub struct Jwt;

impl Jwt {
    pub fn generate_jwt(claims: Claims) -> String {

    let secret = CredentialsManager::get_credentials().unwrap().access_secret().clone();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}
}