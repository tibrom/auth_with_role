use std::collections::HashMap;

use serde_json::Value;
use lazy_static::lazy_static;
use include_dir::{include_dir, Dir};

use super::http::HttpClient;
use super::gql_builder::GqlBuilder;
use super::query_loader::GraphQLDLoader;
use super::errors::{HasuraClientError, HasuraErrorResponse};


const HOST: &str = "https://extrabot.ru";

static GQL_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/gql");
pub const GET_USER_BY_EMAIL: &str = "get_user_by_email.graphql";
pub const GET_USER_BY_ID: &str = "get_user_by_id.graphql";
pub const GET_USER_BY_TG_ID: &str = "get_user_by_tg_id.graphql";


lazy_static! {
    static ref HASURA_CLIENT: HasuraClient = {
        let gql_client = HasuraClientBuilder::new(HOST, GQL_DIR.clone())
            .add_filename(GET_USER_BY_EMAIL)
            .add_filename(GET_USER_BY_ID)
            .add_filename(GET_USER_BY_TG_ID)
            .build();
        gql_client
    };
}

pub struct HasuraClientInterface;

impl HasuraClientInterface {
    pub fn get_hasura_client() -> HasuraClient {
        HASURA_CLIENT.clone()
    }
}


#[derive(Clone, Debug)]
pub struct HasuraClient {
    collection: HashMap<String, GqlBuilder>,
    http: HttpClient,
    
}

impl HasuraClient {
    fn create_http_service(host: &'static str) -> HttpClient {
        let hasura_url: String = host.to_string();
        let mut srv = HttpClient::new(hasura_url);
        //TODO реализовать авторизацию
        //if let Some(key) = api_key {
        //    let mut header_list: Vec<(String, String)> = Vec::new();
        //    header_list.push(("X-Api-Key".to_string(), key));
        //    srv.set_headers(header_list);
        //}
        srv
    }

    pub fn new(host: &'static str,) -> Self {
        let http = Self::create_http_service(host);
        
        Self { 
            collection: HashMap::new(),
            http
        }
    }

    pub fn add_query(&mut self, operation_name: impl ToString, query: impl ToString) {
        self.collection.insert(operation_name.to_string(), GqlBuilder::new(operation_name.to_string(), query.to_string()));
    }

    fn map_gql_error(result: Result<String, reqwest::Error>) -> Result<Value, HasuraClientError> {
        let body = result.map_err(|e|HasuraClientError::HttpRequestError(e))?;

        let value =  serde_json::from_str::<Value>(&body)
            .map_err(|e| HasuraClientError::ResponseJsonParseError(e))?;

        if let Some(e) = value.get("errors") {
            let top_level_error = e.get(0).unwrap();
            let hasura_error_response: HasuraErrorResponse = serde_json::from_value(top_level_error.clone())
                .map_err(|e| HasuraClientError::UnknownHasuraResponseError(e.to_string()))?;
            return Err(HasuraClientError::HasuraResponseError(hasura_error_response));
        }

        Ok(value)
    }

    pub async fn execute(&self, operation_name: impl ToString, variables: Value) -> Result<Value, HasuraClientError> {
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


/// Структура `HasuraClientBuilder` служит для удобной подготовки экземпляра [`HasuraClient`],
/// загружая в него GraphQL-запросы из указанных файлов.
///
/// Позволяет пошагово:
/// - указать директорию, где хранятся `.graphql` или `.gql` файлы;
/// - добавить имена файлов запросов;
/// - автоматически найти и загрузить содержимое запросов в клиента.
///
/// # Пример использования
/// ```ignore
/// let gql_client = HasuraClientBuilder::new("http://localhost:8080".to_string(), None, "queries/")
///     .add_filename("get_users")
///     .add_filename("get_posts")
///     .build();
/// ```
pub struct HasuraClientBuilder<'a> {
    gql_client: HasuraClient,
    filenames: Vec<&'static str>,
    gql_dir: Dir<'a>
}

impl <'a>HasuraClientBuilder<'a> {

    pub fn new(host: &'static str, gql_dir: Dir<'a>) -> Self {
        let gql_client = HasuraClient::new(host);
        Self { gql_client, filenames: Vec::new(), gql_dir}
    }


    pub fn add_filename(&mut self, filename: &'static str) -> &mut Self {
        self.filenames.push(filename);
        self
    }

    /// Ищет все указанные файлы запросов, читает их содержимое и добавляет в клиент.
    ///
    /// Если какой-либо файл не найден, выводится предупреждение через `tracing::warn`.
    fn search_files(&mut self) -> &mut Self {
        println!("self.filenames {:?}", self.filenames);
        let gql_file_service = GraphQLDLoader::new(&self.gql_dir);
        self.filenames.iter().filter_map(|&filename| {
            match gql_file_service.read_query(filename){
                Ok(content) => {Some((filename, content))}
                Err(e) => {
                    tracing::error!("{e}");
                    None
                }
            }
        }).for_each(|(filename, content)| {
            self.gql_client.add_query(filename, content);
        });
        self
    }

    /// Завершает конфигурацию и возвращает склонированный экземпляр [`HasuraClient`].
    ///
    /// # Возвращает
    /// Новый экземпляр [`HasuraClient`] с загруженными запросами.
    pub fn build(&mut self) -> HasuraClient {
        self.search_files();
        self.gql_client.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use include_dir::{include_dir, Dir};
    use serde_json::json;

    static TEST_GQL_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/tests/graphql");
    static TEST_QUERY: &str = "test_query.graphql";

    #[test]
    fn test_graphql_loader_reads_existing_file() {
        let loader = GraphQLDLoader::new(&TEST_GQL_DIR);
        let result = loader.read_query(TEST_QUERY);

        assert!(result.is_ok(), "Expected to read query, but got error");
        let content = result.unwrap();
        assert!(content.contains("query"), "Query should contain the word 'query'");
    }

    #[test]
    fn test_builder_loads_and_adds_queries() {
        let loader = GraphQLDLoader::new(&TEST_GQL_DIR);
        let result = loader.read_query(TEST_QUERY);

        assert!(result.is_ok(), "Expected to read query, but got error");

        let content = result.unwrap();

        let mut builder = HasuraClientBuilder::new("http://localhost", TEST_GQL_DIR.clone());
        builder
            .add_filename(TEST_QUERY);

        let client = builder.build();
        

        let result_gql_builder = client.collection.get(TEST_QUERY);

        assert!(result_gql_builder.is_some(), "Expected to find GqlBuilder, but got error");

        let gql_builder = result_gql_builder.unwrap();

        let query = gql_builder.clone().build();

        let query_json: serde_json::Value = serde_json::from_str(&query)
            .expect("Expected valid JSON from gql_builder.build()");

        let actual_query = query_json.get("query")
            .expect("Missing 'query' field")
            .as_str()
            .expect("'query' is not a string");

        // content может содержать \r\n в Windows, нормализуем
        let normalized_actual = actual_query.replace("\r\n", "\n");
        let normalized_expected = content.replace("\r\n", "\n");

        println!("actual:\n{}", normalized_actual);
        println!("expected:\n{}", normalized_expected);

        assert_eq!(
            normalized_actual.trim(),
            normalized_expected.trim(),
            "Query content mismatch"
        );
    }
}
