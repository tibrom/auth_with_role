use crate::domain::{settings::model::Credentials, user::factories::UserProviderFactory};

use super::user_manager::{UserCommand, UserQuery};


pub struct UserProvider{
    credentials: Credentials,
}
impl UserProvider {
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
}


impl UserProviderFactory for UserProvider {
    type CommandUser = UserCommand;
    type QueryUser = UserQuery;
    fn command_user(&self) -> Self::CommandUser {
        UserCommand::new(self.credentials.clone())
    }
    fn query_user(&self) -> Self::QueryUser {
        UserQuery::new(self.credentials.clone())
    }
}