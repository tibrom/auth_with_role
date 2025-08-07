
static USERNAME: &'static str = "Test";
static APIKEY: &'static str = "FNGFb2Px6cox1wvR98KmU6Fl8IXkBU1x-HWOXQabkZZwCkmKp73YzjVEkKmkgKa9o";
static EMAIL: &'static str = "test@test.test";
static PASSWORD: &'static str = "password";

pub struct MockUser;
impl MockUser {
    pub fn username() -> String {
        USERNAME.to_string()
    }
    pub fn api_key() -> String {
        APIKEY.to_string()
    }
    pub fn email() -> String {
        EMAIL.to_string()
    }
    pub fn password() -> String {
        PASSWORD.to_string()
    }
}