use crate::domain::settings::model::Credentials;
use crate::domain::user::factories::UserProviderFactory;
use crate::infrastructure::network::hasura::client::HasuraClient;
use crate::infrastructure::user::user_manager::{UserCommand, UserQuery};
use crate::mock::http_client::MockHttpClient;


pub struct MockUserProvider {
    credentials: Credentials,
    hasura_client: HasuraClient<MockHttpClient>
}
impl MockUserProvider {
    pub fn new(credentials: Credentials, hasura_client: HasuraClient<MockHttpClient>) -> Self {
        Self { credentials, hasura_client }
    }
}

impl UserProviderFactory for MockUserProvider {
    type CommandUser = UserCommand<MockHttpClient>;
    type QueryUser = UserQuery<MockHttpClient>;
    fn command_user(&self) -> Self::CommandUser {
    
        UserCommand::new(self.credentials.clone(), self.hasura_client.clone())
    }
    fn query_user(&self) -> Self::QueryUser {
        UserQuery::new(self.credentials.clone(), self.hasura_client.clone())
    }
}