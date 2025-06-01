use uuid::Uuid;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct UserForm {
    username: String,
    last_name: Option<String>,
    first_name: Option<String>,
    email: Option<String>,
    telegram_id: Option<String>,
    password_hash: Option<String>,
    aip_key_hash: Option<String>,
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct AllowedRolesForm{
    role: String,
    is_default: bool,
    user_id: Uuid
}