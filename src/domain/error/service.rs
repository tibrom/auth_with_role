use std::error::Error;

pub trait ErrorService {
    fn critical_error(&self, err: &dyn Error) -> String;
    fn not_critical_error(&self, err: &dyn Error) -> String;

}
