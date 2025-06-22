use crate::domain::user::factories::UserProviderFactory;

use super::user_manager::{UserCommand, UserQuery};


pub struct UserProvider;

impl UserProviderFactory for UserProvider {
    type CommandUser = UserCommand;
    type QueryUser = UserQuery;
    fn command_user(&self) -> Self::CommandUser {
        UserCommand
    }
    fn query_user(&self) -> Self::QueryUser {
        UserQuery
    }
}