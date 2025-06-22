use super::service::{QueryUserService, CommandUserService};

pub trait UserProviderFactory {
    type QueryUser: QueryUserService;
    type CommandUser: CommandUserService;

    fn query_user(&self) -> Self::QueryUser;
    fn command_user(&self) -> Self::CommandUser;
}
