use include_dir::{Dir, include_dir};

use crate::infrastructure::network::hasura::client::HasuraClient;
use super::http_client::{MockHttpClient, ResponseMode, ResponseFile};

static RESPONSE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/tests/mock_gql/response/");


#[derive(Clone)]
pub struct MockHasuraClientBuilder {
    http_client: MockHttpClient,
}

impl MockHasuraClientBuilder {
    pub fn new() -> Self {
        let http_client = MockHttpClient::new(ResponseMode::File);
        Self { http_client }
    }

    /// Sets default insert responses: user, roles, auth method, attributes
    pub fn with_user_creation(&mut self) -> &mut Self {
        self.http_client
            .set_file_response(
                "InsertUsersAuthMethod".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "insert_user_auth_method.json"),
            )
            .set_file_response(
                "InsertUsersRole".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "insert_user_role.json"),
            )
            .set_file_response(
                "InsertUser".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "insert_user.json"),
            )
            .set_file_response(
                "InsertMultipleAttributes".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "insert_user_attributes.json"),
            );
        self
    }

    /// Simulates that a user is found by email (overrides API key responses)
    pub fn with_email_auth_method(&mut self) -> &mut Self {
        self.http_client
            .set_file_response(
                "GetAuthMethodByIdentifier".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "query_auth_methods_email.json"),
            )
            .set_file_response(
                "GetAuthMethodByUserId".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "query_auth_methods_email.json"),
            );
        self
    }

    /// Simulates that a user is found by API key (overrides email responses)
    pub fn with_apikey_auth_method(&mut self) -> &mut Self {
        self.http_client
            .set_file_response(
                "GetAuthMethodByIdentifier".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "query_auth_methods_apikey.json"),
            )
            .set_file_response(
                "GetAuthMethodByUserId".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "query_auth_methods_apikey.json"),
            );
        self
    }

    /// Simulates that the auth method exists
    pub fn with_existing_auth_method(&mut self) -> &mut Self {
        self.http_client
            .set_file_response(
                "CheckAuthMethodExists".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "users_auth_method_exist.json"),
            );
        self
    }

    /// Simulates that the auth method does not exist
    pub fn with_nonexistent_auth_method(&mut self) -> &mut Self {
        self.http_client
            .set_file_response(
                "CheckAuthMethodExists".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "users_auth_method_not_exist.json"),
            );
        self
    }

    pub fn with_error_insert_auth_method(&mut self) -> &mut Self {
        self.http_client
            .set_file_response(
                "InsertUsersAuthMethod".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "hasura_error.json"),
            );
        self
    }

    pub fn with_error_insert_role(&mut self) -> &mut Self {
        self.http_client
            .set_file_response(
                "InsertUsersRole".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "hasura_error.json"),
            );
        self
    }

    pub fn with_error_insert_user(&mut self) -> &mut Self {
        self.http_client
            .set_file_response(
                "InsertUser".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "hasura_error.json"),
            );
        self
    }

    pub fn with_error_insert_attributes(&mut self) -> &mut Self {
        self.http_client
            .set_file_response(
                "InsertMultipleAttributes".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "hasura_error.json"),
            );
        self
    }
    //query_auth_methods_empty.json

    pub fn with_auth_method_not_found(&mut self) -> &mut Self {
        self.http_client
            .set_file_response(
                "GetAuthMethodByIdentifier".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "query_auth_methods_empty.json"),
            )
            .set_file_response(
                "GetAuthMethodByUserId".to_string(),
                ResponseFile::new(RESPONSE_DIR.clone(), "query_auth_methods_empty.json"),
            );
        self
    }

    pub fn build(&self) -> HasuraClient<MockHttpClient> {
        HasuraClient::new(Box::new(self.http_client.clone()))
    }
}
