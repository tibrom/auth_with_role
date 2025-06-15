use serde_json::Value;
use std::collections::HashMap;

use crate::domain::jwt::service::{JwtClaimsService as _, TokenService};

use super::errors::{HasuraClientError, HasuraErrorResponse};
use super::gql_builder::GqlBuilder;
use super::http_client::HttpClient;

use super::jwt::claims::ClaimsProvider;
use super::jwt::token::TokenProvider;

#[derive(Clone, Debug)]
pub struct HasuraClient {
    collection: HashMap<String, GqlBuilder>,
    http: HttpClient,
}

impl HasuraClient {
    pub fn new(host: String) -> Result<Self, HasuraClientError> {
        let hasura_url: String = host.to_string();

        let mut srv = HttpClient::new(hasura_url);

        let claims = ClaimsProvider
            .inner_access_claims()
            .map_err(|_| HasuraClientError::CredentialsError)?;
        let token = TokenProvider
            .generate_access(claims)
            .map_err(|_| HasuraClientError::CredentialsError)?;
        
        let mut header_list: Vec<(String, String)> = Vec::new();
        header_list.push(("Authorization".to_string(), format!("Bearer {token}")));
        header_list.push(("content-type".to_string(), "application/json".to_string()));
        srv.set_headers(header_list);

        Ok(Self {
            collection: HashMap::new(),
            http: srv,
        })
    }

    pub fn add_query(&mut self, operation_name: impl ToString, query: impl ToString) {
        self.collection.insert(
            operation_name.to_string(),
            GqlBuilder::new(operation_name.to_string(), query.to_string()),
        );
    }

    fn map_gql_error(result: Result<String, reqwest::Error>) -> Result<Value, HasuraClientError> {
        let body = result.map_err(|e| HasuraClientError::HttpRequestError(e))?;

        let value = serde_json::from_str::<Value>(&body)
            .map_err(|e| HasuraClientError::ResponseJsonParseError(e))?;

        if let Some(e) = value.get("errors") {
            let top_level_error = e.get(0).unwrap();
            let hasura_error_response: HasuraErrorResponse =
                serde_json::from_value(top_level_error.clone())
                    .map_err(|e| HasuraClientError::UnknownHasuraResponseError(e.to_string()))?;
            return Err(HasuraClientError::HasuraResponseError(
                hasura_error_response,
            ));
        }

        Ok(value)
    }

    pub async fn execute(
        &self,
        operation_name: impl ToString,
        variables: Value,
    ) -> Result<Value, HasuraClientError> {
        let operation_name = operation_name.to_string();
        let Some(gql_builder) = self.collection.get(&operation_name) else {
            return Err(HasuraClientError::GqlBuilderNotFound(operation_name));
        };
        let mut gql_builder = gql_builder.clone();

        if let Some(vars) = variables.as_object() {
            for (k, v) in vars {
                gql_builder = gql_builder.variables_add(k.clone(), v.clone());
            }
        }

        let query = gql_builder.build();
        let result = self.http.clone().post(query).await;

        let mut value = Self::map_gql_error(result)?;
        Ok(value["data"].take())
    }
}
