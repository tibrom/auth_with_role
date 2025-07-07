use include_dir::Dir;
use serde::de::DeserializeOwned;
use serde_json::Value;
use super::http_client::HttpClient;
use super::error::HasuraClientError;
use super::gql_descriptor::{ObjectGQLDescriptor, StaticGQLDescriptor};
use crate::infrastructure::hasura::error::HasuraErrorResponse;


/// Клиент для взаимодействия с Hasura GraphQL API.
#[derive(Clone, Debug)]
pub struct HasuraClient {
    hasura_url: String,
    api_key: Option<String>,
    http: HttpClient,
}

impl HasuraClient {
    /// Создаёт новый HasuraClient с заданным URL и (опциональным) API-ключом.
    pub fn new(hasura_url: String, api_key: Option<String>) -> Self {
        let mut srv = HttpClient::new(hasura_url.clone());

        let mut header_list: Vec<(String, String)> = Vec::new();
        header_list.push(("content-type".to_string(), "application/json".to_string()));
        if let Some(token) = api_key.clone() {
            header_list.push(("Authorization".to_string(), format!("Bearer {token}")));
        
        }
        srv.set_headers(header_list);

        Self {
            hasura_url,
            api_key,
            http: srv,
        }
    }

    /// Читает GraphQL-запрос из файла в указанной директории.
    fn read_query(&self, filename: &str, dir: Dir<'static>) -> Result<String, HasuraClientError> {
        match dir.get_file(filename) {
            Some(file) => {
                let content = file
                    .contents_utf8()
                    .ok_or_else(|| HasuraClientError::FailedLoadQuery)?;
                Ok(content.to_string())
            }
            None => Err(HasuraClientError::FailedLoadQuery),
        }
    }

    /// Обрабатывает ответ от Hasura и возвращает JSON-объект либо ошибку.
    fn map_gql_error(&self, result: Result<String, reqwest::Error>) -> Result<Value, HasuraClientError> {
        let body = result.map_err(HasuraClientError::HttpRequestError)?;

        let value = serde_json::from_str::<Value>(&body)
            .map_err(|e| HasuraClientError::ResponseJsonParseError(e.to_string()))?;

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

    /// Универсальное выполнение GraphQL запроса.
    ///
    /// `D`: пользовательская структура, реализующая оба трейта:
    /// - `StaticGQLDescriptor` — описывает мета-данные о GQL-файле.
    /// - `ObjectGQLDescriptor<T>` — описывает переменные и десериализацию.
    pub async fn execute<D, T>(&self, descriptor: &D) -> Result<T, HasuraClientError>
    where
        D: StaticGQLDescriptor + ObjectGQLDescriptor,
        T: DeserializeOwned {
            let dir = D::path();
            let filename = D::filename();
            let operation_name = D::operation_name();
            let query = self.read_query(filename, dir)?;

            let value = serde_json::json!({
                "operationName": operation_name,
                "query": query,
                "variables": descriptor.variables()
            });

            let http_result = self.http.clone().post(value.to_string()).await;
            let result_value = self.map_gql_error(http_result)?;

            let result: T = serde_json::from_value(result_value["data"].clone()).map_err(|e| e.to_string())
                .map_err(HasuraClientError::ResponseJsonParseError)?;
            Ok(result)
        }
}
