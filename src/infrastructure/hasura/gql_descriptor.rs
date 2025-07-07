use include_dir::Dir;
use serde_json::Value;


pub trait StaticGQLDescriptor {
    fn filename() -> &'static str;
    fn operation_name() -> &'static str;
    fn path() -> Dir<'static>;
}


pub trait ObjectGQLDescriptor {
    fn variables(&self) -> Value {
        Value::Null
    }
}