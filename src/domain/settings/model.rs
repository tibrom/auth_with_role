use getset::{Getters, Setters};


#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Default)]
pub struct Credentials {
    #[get = "pub"]
    new_user_role: String,
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
    hasura_credentials: HasuraCredentials
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