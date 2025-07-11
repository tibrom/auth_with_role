use super::service::{QueryUserService, CommandUserService};

pub trait UserProviderFactory {
    type QueryUser: QueryUserService + Send;
    type CommandUser: CommandUserService + Send;

    fn query_user(&self) -> Self::QueryUser;
    fn command_user(&self) -> Self::CommandUser;
}
