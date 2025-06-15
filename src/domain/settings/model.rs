use getset::{Getters, Setters};


#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Default)]
pub struct Credentials {
    #[get = "pub"]
    host: String,
    #[get = "pub"]
    port: u16,
    #[get = "pub"]
    expiration_access_hours: i16,
    #[get = "pub"]
    expiration_refresh_hours: i16,
    #[get = "pub"]
    access_secret: String,
    #[get = "pub"]
    refresh_secret: String,
    #[get = "pub"]
    hasura_url: String,
    #[get = "pub"]
    hasura_credentials: HasuraCredentials,
    #[get = "pub"]
    new_user_role: NewUserRole,
    #[get = "pub"]
    api_key_length: u16,
    #[get = "pub"]
    encryption_api_key: String
}

#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Default)]
pub struct HasuraCredentials{
    #[get = "pub"]
    x_hasura_default_role: String,
    #[get = "pub"]
    exp: i16, //hours
    #[get = "pub"]
    x_hasura_user_id: String
}


#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Default)]
pub struct NewUserRole{
    #[get = "pub"]
    with_email: String,
    #[get = "pub"]
    with_telegram: String,
}