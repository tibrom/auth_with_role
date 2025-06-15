use super::model::Credentials;
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
pub trait CredentialsService {
    type Error: AppErrorInfo;

    fn get_credentials(&self) -> Result<Credentials, Self::Error>;

}