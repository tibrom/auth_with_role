use std::collections::HashMap;
use std::fmt::Debug;

use super::error::{HasuraClientError, HasuraError};
use super::interface::{HasuraInterface, ObjectGQLDescriptor, StaticGQLDescriptor};
use super::HttpClientInterface;
use include_dir::Dir;
use serde::de::DeserializeOwned;
use serde_json::{value, Value};

/// Клиент для взаимодействия с Hasura GraphQL API.
#[derive(Clone, Debug)]
pub struct HasuraClient<T: HttpClientInterface> {
    http: Box<T>,
    pub query_hash: HashMap<String, String>,
}

impl<T: HttpClientInterface + Clone> HasuraClient<T> {
    /// Создаёт новый HasuraClient с заданным URL и (опциональным) API-ключом.
    pub fn new(http: Box<T>) -> Self {
        Self {
            http,
            query_hash: HashMap::new(),
        }
    }

    fn get_hash_key(&self, filename: &str, dir: &Dir<'static>) -> String {
        if let Some(s) = dir.path().to_str() {
            return format!("{}/{}", s, filename);
        };
        filename.to_string()
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

    fn get_query(
        &mut self,
        filename: &str,
        dir: Dir<'static>,
    ) -> Result<String, HasuraClientError> {
        let key = self.get_hash_key(filename, &dir);
        println!("key {:?}", key);
        if let Some(query) = self.query_hash.get(&key) {
            return Ok(query.clone());
        };
        let query = self.read_query(filename, dir)?;
        self.query_hash.insert(key, query.clone());
        Ok(query)
    }

    /// Обрабатывает ответ от Hasura и возвращает JSON-объект либо ошибку.
    fn map_gql_error(&self, result: String) -> Result<Value, HasuraClientError> {
        let value = serde_json::from_str::<Value>(&result)
            .map_err(|e| HasuraClientError::ResponseJsonParseError(e.to_string()))?;

        if let Some(e) = value.get("errors") {
            let top_level_error = e.get(0).unwrap();
            let hasura_error_response: HasuraError =
                serde_json::from_value(top_level_error.clone())
                    .map_err(|e| HasuraClientError::UnknownHasuraResponseError(e.to_string()))?;
            return Err(HasuraClientError::HasuraResponseError(
                hasura_error_response,
            ));
        }

        Ok(value)
    }

    fn create_body(&mut self, operation_name: &'static str, query: String, variables: serde_json::Value) -> serde_json::Value {
        serde_json::json!({
            "operationName": operation_name,
            "query": query,
            "variables": variables
        })
    }
}

impl<R: HttpClientInterface + Clone> HasuraInterface for HasuraClient<R> {
    async fn execute<D, T>(&mut self, descriptor: &D) -> Result<T, HasuraClientError>
    where
        D: StaticGQLDescriptor + ObjectGQLDescriptor + Sync,
        T: DeserializeOwned + Send,
    {
        let dir = descriptor.path();
        let filename = descriptor.filename();
        let operation_name = descriptor.operation_name();
        let query = self.get_query(filename, dir)?;
        let variables = descriptor.variables();
        
        let value = self.create_body(operation_name, query, variables);


        let http_result = self
            .http
            .clone()
            .post(value.to_string())
            .await
            .map_err(|e| HasuraClientError::HttpRequestError(e.to_string()))?;
        let result_value = self.map_gql_error(http_result)?;

        let result: T = serde_json::from_value(result_value["data"].clone())
            .map_err(|e| e.to_string())
            .map_err(HasuraClientError::ResponseJsonParseError)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::network::hasura::interface::{
        ObjectGQLDescriptor, StaticGQLDescriptor,
    };
    use crate::mock::http_client::{MockHttpClient, ResponseFile, ResponseMode};
    use include_dir::{include_dir, Dir};
    use serde::{Deserialize, Serialize};

    static GQL_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/tests/mock_gql/query/");
    static RESPONSE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/tests/mock_gql/response/");
    
    static QUERY_NAME: &'static str = "TestQuery";
   

    fn create_body(query_name: &str) -> serde_json::Value {
        serde_json::json!({
            "operationName": QUERY_NAME,
            "query": "TEST",
            "variables": {
                "data": 1
            }
        })
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
    struct TestValue {
        str_value: String,
        u64_value: u64,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
    struct ReqwestValue;

    impl ObjectGQLDescriptor for ReqwestValue {
        fn variables(&self) -> serde_json::Value {
            serde_json::json!({ "data": 1 })
        }
    }

    impl StaticGQLDescriptor for ReqwestValue {
        fn filename(&self) -> &'static str {
            "test_query.graphql"
        }
        fn operation_name(&self) -> &'static str {
            QUERY_NAME
        }
        fn path(&self) -> Dir<'static> {
            GQL_DIR.clone()
        }
    }

    #[tokio::test]
    async fn correct_response_with_cash() {
    
        let query = "TEST QUERY".to_string();
        let response = serde_json::json!({
            "data": {
                "str_value": "TEST",
                "u64_value": 12
            }
        });

        let mock_http = MockHttpClient::new(ResponseMode::Memory)
            .set_memory_response(QUERY_NAME.to_string(), response).clone();

        let mut hasura_client = HasuraClient::new(Box::new(mock_http));
        let descriptor = ReqwestValue;

        let key_hash = hasura_client.get_hash_key(descriptor.filename(), &descriptor.path());
        hasura_client.query_hash.insert(key_hash, query);

        let r = hasura_client
            .execute::<ReqwestValue, TestValue>(&descriptor)
            .await;
        assert!(r.is_ok());
        let result = r.unwrap();
        assert_eq!(result.str_value, "TEST");
        assert_eq!(result.u64_value, 12);
    }

    #[tokio::test]
    async fn load_query_from_file() {
        let response = serde_json::json!({
            "data": {
                "str_value": "TEST",
                "u64_value": 12
            }
        });

        let assert_query = "{\"operationName\":\"TestQuery\",\"query\":\"query TestQuery {\\n    ping\\n}\",\"variables\":{\"data\":1}}".to_string();

        let mock_http = MockHttpClient::new(ResponseMode::Memory)
            .set_memory_response(QUERY_NAME.to_string(), response).clone();
        let recorder = mock_http.recorder();
        let mut hasura_client = HasuraClient::new(Box::new(mock_http));
        let descriptor = ReqwestValue;

        let r = hasura_client
            .execute::<ReqwestValue, TestValue>(&descriptor)
            .await;

        let query = recorder.read_data().await.unwrap();

        assert_eq!(query, assert_query);
        println!("result  query {:?}", query);
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_query_hash() {
        let query = "TEST QUERY".to_string();
        let response = serde_json::json!({
            "data": {
                "str_value": "TEST",
                "u64_value": 12
            }
        });

        let mock_http = MockHttpClient::new(ResponseMode::Memory)
            .set_memory_response(QUERY_NAME.to_string(), response).clone();
        let recorder = mock_http.recorder();

        let mut hasura_client = HasuraClient::new(Box::new(mock_http));
        let descriptor = ReqwestValue;

        let key_hash = hasura_client.get_hash_key(descriptor.filename(), &descriptor.path());
        hasura_client.query_hash.insert(key_hash, query.clone());

        let r = hasura_client
            .execute::<ReqwestValue, TestValue>(&descriptor)
            .await;
        assert!(r.is_ok());

        let assert_query =
            "{\"operationName\":\"TestQuery\",\"query\":\"TEST QUERY\",\"variables\":{\"data\":1}}"
                .to_string();
        let query = recorder.read_data().await.unwrap();

        assert_eq!(query, assert_query);
    }

    #[tokio::test]
    async fn correct_parse_hasura_error(){
        let response_file = "hasura_error.json";
        let response= ResponseFile::new(RESPONSE_DIR.clone(), response_file);

        let mock_http = MockHttpClient::new(ResponseMode::File)
            .set_file_response(QUERY_NAME.to_string(), response).clone();

        let mut hasura_client = HasuraClient::new(Box::new(mock_http));

        let descriptor = ReqwestValue;

        let r = hasura_client
            .execute::<ReqwestValue, TestValue>(&descriptor)
            .await;

        assert!(r.is_err());
        let error = r.err().unwrap();
        let is_correct_error = match error {
            HasuraClientError::HasuraResponseError(_) => true,
            _ => false
        };

        assert!(is_correct_error);
    
    }

    #[tokio::test]
    async fn test_body(){
        
        let operation_name = ReqwestValue.operation_name();
        let query = "TEST".to_string();
        let variables = ReqwestValue.variables();

        let mock_http = MockHttpClient::new(ResponseMode::Memory);
       

        let mut hasura_client = HasuraClient::new(Box::new(mock_http));

        let body = hasura_client.create_body(operation_name, query, variables);

        let accept_body = create_body(QUERY_NAME);

        assert_eq!(body, accept_body);
    }

}
