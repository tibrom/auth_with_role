use crate::application::sign_up_usecase::dto::SignUpRequestDto;
use crate::application::sign_up_usecase::dto::UserDataDto;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct ApiKeyDto {
    pub api_key: String
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
}
impl UserDTO {
    pub fn to_user_data_dto(&self) -> UserDataDto {
        UserDataDto { username: self.username.clone(), email: self.email.clone() }
    }
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct SignUpUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}


impl From<SignUpRequestDto> for SignUpUserDto {
    fn from(value: SignUpRequestDto) -> Self {
        Self { username: value.username, email: value.email, password: value.password }
    }
}


