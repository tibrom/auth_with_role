
use crate::domain::jwt::model::{Claims, HasuraClaims, RefreshClaims};
use crate::domain::jwt::service::JwtClaimsService;
use crate::domain::settings::service::CredentialsService as _;
use crate::domain::user::model::UserWithRole;

use super::config::credentials_provider::CredentialsProvider;
use super::error::JwtError;

pub struct ClaimsProvider;

impl JwtClaimsService for ClaimsProvider {
    type Error = JwtError;

    fn access_claims(&self, user: &UserWithRole) -> Result<Claims, Self::Error> {
        let x_hasura_default_role = user
            .allowed_roles()
            .iter()
            .find(|v| *v.is_default())
            .map(|v| Ok(v.role().clone()))
            .unwrap_or_else(|| Err(JwtError::DefaultRoleMissing))?;

        let x_hasura_allowed_roles = user
            .allowed_roles()
            .iter()
            .map(|v| v.role().clone())
            .collect::<Vec<_>>();

        let x_hasura_user_id = user.id().to_string();

        let hasura_claims = HasuraClaims::new(
            x_hasura_default_role,
            x_hasura_allowed_roles,
            x_hasura_user_id.clone(),
        );
        let exp = CredentialsProvider
            .get_credentials()
            .map(|v| v.expiration_access_hours().clone())
            .map_err(|e| JwtError::CredentialsUnavailable(e))?;

        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(exp.into()))
            .expect("valid timestamp")
            .timestamp() as usize;

        Ok(Claims::new(
            x_hasura_user_id,
            false,
            expiration,
            hasura_claims,
        ))
    }

    fn inner_access_claims(&self) -> Result<Claims, Self::Error> {
        let hasura_credentials = CredentialsProvider
            .get_credentials()
            .map(|e| e.hasura_credentials().clone())
            .map_err(|e| JwtError::CredentialsUnavailable(e))?;

        let x_hasura_default_role = hasura_credentials.x_hasura_default_role().clone();
        let x_hasura_allowed_roles = vec![x_hasura_default_role.clone()];
        let x_hasura_user_id = hasura_credentials.x_hasura_user_id().clone();
        let exp = hasura_credentials.exp().clone();
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(exp.into()))
            .expect("valid timestamp")
            .timestamp() as usize;

        let hasura_claims = HasuraClaims::new(
            x_hasura_default_role,
            x_hasura_allowed_roles,
            x_hasura_user_id.clone(),
        );

        Ok(Claims::new(
            x_hasura_user_id,
            false,
            expiration,
            hasura_claims,
        ))
    }

    fn refresh_claims(&self, user: &UserWithRole) -> Result<RefreshClaims, Self::Error> {
        let sub = user.id().to_string();
        let exp = CredentialsProvider
            .get_credentials()
            .map(|v| v.expiration_refresh_hours().clone())
            .map_err(|e| JwtError::CredentialsUnavailable(e))?;
        Ok(RefreshClaims::new(sub, exp))
    }
}
