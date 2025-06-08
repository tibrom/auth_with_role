use std::fmt;
use thiserror::Error;

use crate::conf::errors::CredentialsError;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Failed getting credentials {0}")]
    CredentialsNotFound(#[from] CredentialsError),
    #[error("Missing default role")]
    DefaultRoleNotFound,
}
