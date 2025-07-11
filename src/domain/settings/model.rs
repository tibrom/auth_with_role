use getset::{Getters, Setters};

#[derive(
    Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Default,
)]
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
}

impl Credentials {
    pub fn mock() -> Self {
        let new_user_role = NewUserRole{
            with_email: "TEST".to_string(),
            with_telegram: "@TEST".to_string()
        };

        let hasura_credentials = HasuraCredentials {
            x_hasura_default_role: "TEST".to_string(),
            exp: 1,
            x_hasura_user_id: "TEST".to_string()
        };
        Self { 
            host: "TEST_HOST".to_string(),
            port: 100,
            expiration_access_hours: 100,
            expiration_refresh_hours: 100,
            access_secret: "TEST_ACCESS".to_string(),
            refresh_secret: "REFRESH_TEST".to_string(),
            hasura_url: "URL".to_string(),
            hasura_credentials,
            new_user_role,
            api_key_length: 32
        }

    }
}


#[derive(
    Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Default,
)]
pub struct HasuraCredentials {
    #[get = "pub"]
    x_hasura_default_role: String,
    #[get = "pub"]
    exp: i16, //hours
    #[get = "pub"]
    x_hasura_user_id: String,
}

#[derive(
    Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Default,
)]
pub struct NewUserRole {
    #[get = "pub"]
    with_email: String,
    #[get = "pub"]
    with_telegram: String,
}
