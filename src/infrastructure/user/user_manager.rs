use crate::domain::settings::model::Credentials;
use crate::domain::user::service::{CommandUserService, QueryUserService};
use crate::infrastructure::network::client_manager::HasuraClientManager;
use crate::infrastructure::network::hasura::interface::HasuraInterface;
use crate::infrastructure::network::hasura::client::HasuraClient;
use crate::infrastructure::network::http::client::HttpClient;
use crate::infrastructure::network::http::interface::HttpClientInterface;

use super::errors::UserManagerError;
use super::requests::add_auth_method::{AddAuthMethodDescriptor, AddAuthMethodResponse};
use super::requests::add_roles::{AddRoleRequestDescriptor, AddRoleResponse};
use super::requests::add_user::{AddUserRequestDescriptor, AddUserResponse};
use super::requests::add_user_attribute::{AddAttributesRequestDescriptor, AddAttributesResponse};
use super::requests::check_auth_method::{
    CheckAuthMethodRequestDescriptor, CheckAuthMethodResponse,
};

use crate::domain::user::models::base::{AuthMethod, User, UserAttribute, UserRole};

pub struct UserCommand<T: HttpClientInterface> {
    credentials: Credentials,
    hasura_client: HasuraClient<T>
}

impl <T: HttpClientInterface + Clone> UserCommand <T> {
    pub fn new(credentials: Credentials, hasura_client: HasuraClient<T>) -> Self {
        Self { credentials, hasura_client }
    }
}

impl <T: HttpClientInterface + Clone> CommandUserService for UserCommand <T> {
    type Error = UserManagerError;

    async fn auth_identifier_is_free(&self, identifier: String, auth_type:&str ) -> Result<bool, Self::Error> {
        let mut client = self.hasura_client.clone();

        let descriptor = CheckAuthMethodRequestDescriptor::new(identifier, auth_type.to_owned());

        
        let result = client
            .execute::<CheckAuthMethodRequestDescriptor, CheckAuthMethodResponse>(&descriptor)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        Ok(result.users_auth_method_aggregate.aggregate.count == 0)
    }

    async fn add_auth_method(&self, auth_method: AuthMethod) -> Result<AuthMethod, Self::Error> {
        let mut client = self.hasura_client.clone();

        let descriptor = AddAuthMethodDescriptor::new(auth_method);

        let result = client
            .execute::<AddAuthMethodDescriptor, AddAuthMethodResponse>(&descriptor)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        match result.insert_users_auth_method.returning.first() {
            Some(user) => Ok(user.clone()),
            None => Err(UserManagerError::FailedCreateUser),
        }
    }
    async fn add_role(&self, user_role: UserRole) -> Result<UserRole, Self::Error> {
        let mut client = self.hasura_client.clone();

        let descriptor = AddRoleRequestDescriptor::new(user_role);

        let result = client
            .execute::<AddRoleRequestDescriptor, AddRoleResponse>(&descriptor)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        match result.insert_users_user_role.returning.first() {
            Some(user) => Ok(user.clone()),
            None => Err(UserManagerError::FailedCreateUser),
        }
    }
    async fn add_user(&self) -> Result<User, Self::Error> {
        let mut client = self.hasura_client.clone();

        let descriptor = AddUserRequestDescriptor;

        let result = client
            .execute::<AddUserRequestDescriptor, AddUserResponse>(&descriptor)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        match result.insert_users_user.returning.first() {
            Some(user) => Ok(user.clone()),
            None => Err(UserManagerError::FailedCreateUser),
        }
    }
    async fn add_user_attribute(
        &self,
        attributes: Vec<UserAttribute>,
    ) -> Result<Vec<UserAttribute>, Self::Error> {
        let mut client = self.hasura_client.clone();

        let descriptor = AddAttributesRequestDescriptor::new(attributes);

        let result = client
            .execute::<AddAttributesRequestDescriptor, AddAttributesResponse>(&descriptor)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        Ok(result.insert_users_user_attribute.returning)
    }
}

use crate::domain::user::models::extended::ExtendedAuthMethod;

use super::requests::get_user_by_id::{
    GetUserByByUserIdResponse, GetUserByUserIdRequestDescriptor,
};
use super::requests::get_user_by_identifier::{
    GetUserByByIdentifierResponse, GetUserByIdentifierRequestDescriptor,
};

pub struct UserQuery<T: HttpClientInterface> {
    credentials: Credentials,
    hasura_client: HasuraClient<T>
}

impl <T: HttpClientInterface + Clone> UserQuery<T> {
    pub fn new(credentials: Credentials, hasura_client: HasuraClient<T>) -> Self {
        Self { credentials,  hasura_client}
    }
}

impl <T: HttpClientInterface + Clone> QueryUserService for UserQuery<T> {
    type Error = UserManagerError;

    async fn get_user_by_identifier(
        &self,
        identifier: &str,
        auth_type: &str,
    ) -> Result<Option<ExtendedAuthMethod>, Self::Error> {
        let mut client = self.hasura_client.clone();

        let descriptor = GetUserByIdentifierRequestDescriptor::new(identifier.to_owned(), auth_type.to_owned());

        let result = client
            .execute::<GetUserByIdentifierRequestDescriptor, GetUserByByIdentifierResponse>(
                &descriptor,
            )
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        Ok(result
            .users_auth_method
            .first()
            .and_then(|v| Some(v.clone())))
    }

    async fn get_user_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Vec<ExtendedAuthMethod>, Self::Error> {
        let mut client = self.hasura_client.clone();
        let descriptor = GetUserByUserIdRequestDescriptor::new(id.to_owned());

        let result = client
            .execute::<GetUserByUserIdRequestDescriptor, GetUserByByUserIdResponse>(&descriptor)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        Ok(result.users_auth_method)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use include_dir::{include_dir, Dir};
    use uuid::Uuid;

    use crate::infrastructure::network::hasura::client::HasuraClient;
    use crate::mock::http_client::{MockHttpClient, ResponseMode, ResponseFile};

    static RESPONSE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/tests/mock_gql/response/");

    fn create_mock_http_client(query_name: String, response_file: &'static str) -> MockHttpClient {
        let response = ResponseFile::new(RESPONSE_DIR.clone(), response_file);
        let mut client = MockHttpClient::new(ResponseMode::File);
        client.set_file_response(query_name.clone(), response);
        client.clone()
    }

    fn mock_credentials() -> Credentials {
        Credentials::mock()
    }


    #[tokio::test]
    async fn user_command_auth_identifier_is_free(){
        let query_name = "CheckAuthMethodExists";
        let response_file = "users_auth_method_not_exist.json";
        let identifier = "TEST".to_string();
        let http_client = create_mock_http_client(query_name.to_owned(), response_file);
        let hasura_client = HasuraClient::new(Box::new(http_client));
        let credentials = mock_credentials();
        let user_command = UserCommand::new(credentials, hasura_client);

        let result = user_command.auth_identifier_is_free(identifier, "email").await;

        assert!(result.is_ok());
        assert!(result.unwrap())
    }

    #[tokio::test]
    async fn user_command_add_auth_method() {
        let query_name = "InsertUsersAuthMethod";
        let response_file = "insert_user_auth_method.json";
        let auth_method = AuthMethod::new(
            Uuid::new_v4(),
            "test".to_string(),
            "test@test.com".to_string(),
            Some("test_secret".to_string())
        );
        let http_client = create_mock_http_client(query_name.to_owned(), response_file);
        let hasura_client = HasuraClient::new(Box::new(http_client));
        let credentials = mock_credentials();
        let user_command = UserCommand::new(credentials, hasura_client);

        let result = user_command.add_auth_method(auth_method).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn user_command_add_role(){
        let query_name = "InsertUsersRole";
        let response_file = "insert_user_role.json";
        let user_role = UserRole::new(true, "test".to_string(), Uuid::new_v4());
        let http_client = create_mock_http_client(query_name.to_owned(), response_file);
        let hasura_client = HasuraClient::new(Box::new(http_client));
        let credentials = mock_credentials();
        let user_command = UserCommand::new(credentials, hasura_client);
        
        let result = user_command.add_role(user_role).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn user_command_add_user() {
        let query_name = "InsertUser";
        let response_file = "insert_user.json";
        let http_client = create_mock_http_client(query_name.to_owned(), response_file);
        let hasura_client = HasuraClient::new(Box::new(http_client));
        let credentials = mock_credentials();
        let user_command = UserCommand::new(credentials, hasura_client);
        
        let result = user_command.add_user().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn user_command_add_user_attribute(){
        let query_name = "InsertMultipleAttributes";
        let response_file = "insert_user_attributes.json";
        let http_client = create_mock_http_client(query_name.to_owned(), response_file);
        let hasura_client = HasuraClient::new(Box::new(http_client));
        let credentials = mock_credentials();
        let user_command = UserCommand::new(credentials, hasura_client);
        let user_attribute = UserAttribute::new(
            Uuid::new_v4(),
            "test".to_string(),
            "test".to_string()
        );

        let attributes = vec![user_attribute];

        let result = user_command.add_user_attribute(attributes).await;

        assert!(result.is_ok());

    }

    #[tokio::test]
    async fn query_user_get_user_by_identifier (){
        let query_name = "GetAuthMethodByIdentifier";
        let response_file = "query_auth_methods_email.json";
        let identifier = "TEST".to_string();
        let http_client = create_mock_http_client(query_name.to_owned(), response_file);
        let hasura_client = HasuraClient::new(Box::new(http_client));
        let credentials = mock_credentials();
        let user_command = UserQuery::new(credentials, hasura_client);

        let result = user_command.get_user_by_identifier(&identifier, "email").await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }


    #[tokio::test]
    async fn query_user_get_user_by_id (){
        let query_name = "GetAuthMethodByUserId";
        let response_file = "query_auth_methods_email.json";
        let id = Uuid::new_v4();
        let http_client = create_mock_http_client(query_name.to_owned(), response_file);
        let hasura_client = HasuraClient::new(Box::new(http_client));
        let credentials = mock_credentials();
        let user_command = UserQuery::new(credentials, hasura_client);

        let result = user_command.get_user_by_id(id).await;

        assert!(result.is_ok());
        assert!(result.unwrap().first().is_some());
    }



            
}