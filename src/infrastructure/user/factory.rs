use crate::domain::{settings::model::Credentials, user::factories::UserProviderFactory};
use crate::infrastructure::network::hasura::client::HasuraClient;
use crate::infrastructure::network::http::client::HttpClient;
use crate::infrastructure::network::http::interface::HttpClientInterface;




use super::user_manager::{UserCommand, UserQuery};

pub struct UserProvider {
    credentials: Credentials,
    hasura_client: HasuraClient<HttpClient>
}
impl UserProvider {
    pub fn new(credentials: Credentials, hasura_client: HasuraClient<HttpClient>) -> Self {
        Self { credentials, hasura_client }
    }
}

impl UserProviderFactory for UserProvider {
    type CommandUser = UserCommand<HttpClient>;
    type QueryUser = UserQuery<HttpClient>;
    fn command_user(&self) -> Self::CommandUser {
        UserCommand::new(self.credentials.clone(), self.hasura_client.clone())
    }
    fn query_user(&self) -> Self::QueryUser {
        UserQuery::new(self.credentials.clone(), self.hasura_client.clone())
    }
}
