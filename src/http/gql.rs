use super::errors::{HasuraClientError, HasuraErrorResponse};
use super::http::HttpClient;
use serde_json::{json, Value};
use std::{collections::HashMap, result, sync::Arc};

#[derive(Debug, Clone)]
pub struct GqlBuilder {
    operation_name: String,
    query: String,
    variables: HashMap<String, Value>,
}

impl GqlBuilder {
    pub fn new(operation_name: String, query: String) -> Self {
        Self {
            operation_name,
            query,
            variables: HashMap::new(),
        }
    }

    pub fn variables_add(mut self, key: String, val: Value) -> Self {
        self.variables.insert(key, val);
        self
    }

    pub fn build(&mut self) -> String {
        let val = json!({
            "operationName": self.operation_name,
            "query": self.query,
            "variables": self.variables.clone()
        });
        let request = serde_json::to_string(&val).unwrap();
        // Clean variables for next request
        self.variables.clear();
        request
    }
}
