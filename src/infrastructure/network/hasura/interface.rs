use serde::de::DeserializeOwned;
use include_dir::Dir;
use serde_json::Value;
use super::error::HasuraClientError;


pub trait HasuraInterface {
    async fn execute<D, T>(&mut self, descriptor: &D) -> Result<T, HasuraClientError>
    where
        D: StaticGQLDescriptor + ObjectGQLDescriptor + Sync,
        T: DeserializeOwned + Send;
}

pub trait StaticGQLDescriptor {
    fn filename(&self) -> &'static str;
    fn operation_name(&self) -> &'static str;
    fn path(&self) -> Dir<'static>;
}


pub trait ObjectGQLDescriptor {
    fn variables(&self) -> Value {
        Value::Null
    }
}