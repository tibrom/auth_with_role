#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct UserForm {
    id: Uuid,
    name: String,
    email: String,
    tg_id: String,
    password_hash: String,
}



#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct AllowedRolesForm{
    id: Uuid,
    role: String,
    is_default: Boolean,
    created_at: DateTime,
    user_id: Uuid
}
