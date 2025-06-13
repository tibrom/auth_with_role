use super::model::Credentials;
pub trait CredentialsService {
    type Error;

    fn get_credentials(&self) -> Result<Credentials, Self::Error>;

}