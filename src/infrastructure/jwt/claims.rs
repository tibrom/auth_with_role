use crate::domain::jwt::model::{Claims, HasuraClaims, RefreshClaims};
use crate::domain::jwt::service::JwtClaimsService;
use crate::domain::settings::model::Credentials;
use crate::domain::user::models::extended::ExtendedAuthMethod;

use super::error::JwtError;

pub struct ClaimsProvider {
    credentials: Credentials,
}

impl ClaimsProvider {
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
}

impl JwtClaimsService for ClaimsProvider {
    type Error = JwtError;

    fn access_claims(&self, user: &ExtendedAuthMethod) -> Result<Claims, Self::Error> {
        let x_hasura_default_role = user
            .user()
            .user_roles()
            .iter()
            .find(|v| *v.is_default())
            .map(|v| Ok(v.role().clone()))
            .unwrap_or_else(|| Err(JwtError::DefaultRoleMissing))?;

        let x_hasura_allowed_roles = user
            .user()
            .user_roles()
            .iter()
            .map(|v| v.role().clone())
            .collect::<Vec<_>>();

        let x_hasura_user_id = user.id().to_string();

        let hasura_claims = HasuraClaims::new(
            x_hasura_default_role,
            x_hasura_allowed_roles,
            x_hasura_user_id.clone(),
        );
        let exp = self.credentials.expiration_access_hours().clone();

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
        let hasura_credentials = self.credentials.hasura_credentials().clone();

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

    fn refresh_claims(&self, user: &ExtendedAuthMethod) -> Result<RefreshClaims, Self::Error> {
        let sub = user.user_id().to_string();
        let exp = self.credentials.expiration_refresh_hours().clone();
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(exp.into()))
            .expect("valid timestamp")
            .timestamp() as usize;
        Ok(RefreshClaims::new(sub, expiration))
    }
}


#[cfg(test)]
mod test {
    use crate::infrastructure::jwt::claims;

    use super::*;


    fn extended_auth_method() -> ExtendedAuthMethod {
        ExtendedAuthMethod::mock()
    }

    #[test]
    fn access_claims() {
        let credentials = Credentials::mock();
        let user = extended_auth_method();
        let provider = ClaimsProvider::new(credentials.clone());
        let claims_result = provider.access_claims(&user);
        assert!(claims_result.is_ok());
        let claims = claims_result.unwrap();
        let allowed_role = user.user()
            .user_roles()
            .iter()
            .map(|v|v.role())
            .cloned()
            .collect::<Vec<_>>();

        let claims_role = claims.hasura_claims.x_hasura_allowed_roles.clone();

        for role in &allowed_role {
            assert!(
                claims_role.contains(role),
                "Role `{}` from allowed_role not found in claims_role",
                role
            );
        }

        let default_claims_role = claims.hasura_claims.x_hasura_default_role.clone();
        assert!(allowed_role.contains(&default_claims_role));

        let default_role = user
            .user()
            .user_roles()
            .iter()
            .find(|v|v.role() == &default_claims_role);

        assert!(default_role.is_some());
        assert!(default_role.unwrap().is_default());
        let claims_user_id = claims.hasura_claims.x_hasura_user_id.clone();

        assert_eq!(user.id().to_string(), claims_user_id);

        let expected_duration = credentials.expiration_access_hours().clone();
        let now_timestamp = chrono::Utc::now().timestamp() as usize;
        let expected_exp = now_timestamp + (expected_duration as usize) * 3600;

        
        let diff = if claims.exp > expected_exp {
            claims.exp - expected_exp
        } else {
            expected_exp - claims.exp
        };
        assert!(
            diff <= 30,
            "Expiration timestamp differs by more than 30 seconds: expected {}, got {}",
            expected_exp,
            claims.exp
        );

        assert!(!claims.admin);

    }

    #[test]
    fn inner_access_claims() {
        let credentials = Credentials::mock();
        let provider = ClaimsProvider::new(credentials.clone());
        let claims_result = provider.inner_access_claims();
        assert!(claims_result.is_ok());
        let claims = claims_result.unwrap();
        let assert_role = credentials.hasura_credentials().x_hasura_default_role().clone();
        let assert_user_id = credentials.hasura_credentials().x_hasura_user_id();

        let claims_role = claims.hasura_claims.x_hasura_allowed_roles.clone();
        assert_eq!(claims_role.len(), 1);
        assert_eq!(*claims_role.first().unwrap(), assert_role);
        assert_eq!(*claims.hasura_claims.x_hasura_user_id, *assert_user_id);

        let expected_duration = credentials.hasura_credentials().exp().clone();
        let now_timestamp = chrono::Utc::now().timestamp() as usize;
        let expected_exp = now_timestamp + (expected_duration as usize) * 3600;

        
        let diff = if claims.exp >= expected_exp {
            claims.exp - expected_exp
        } else {
            expected_exp - claims.exp
        };
        assert!(
            diff <= 30,
            "Expiration timestamp differs by more than 30 seconds: expected {}, got {} diff {}",
            expected_exp,
            claims.exp,
            diff
        );
        assert!(!claims.admin)
    }

    #[test]
    fn refresh_claims(){
        let credentials = Credentials::mock();
        let user = extended_auth_method();
        let provider = ClaimsProvider::new(credentials.clone());
        let claims_result = provider.refresh_claims(&user);
        assert!(claims_result.is_ok());
        let claims = claims_result.unwrap();
        assert_eq!(user.user_id().to_string(), claims.sub);


        let expected_duration = credentials.expiration_refresh_hours().clone();
        let now_timestamp = chrono::Utc::now().timestamp() as usize;
        let expected_exp = now_timestamp + (expected_duration as usize) * 3600;

        
        let diff = if claims.exp > expected_exp {
            claims.exp - expected_exp
        } else {
            expected_exp - claims.exp
        };
        assert!(
            diff <= 30,
            "Expiration timestamp differs by more than 30 seconds: expected {}, got {}",
            expected_exp,
            claims.exp
        );
    }

}