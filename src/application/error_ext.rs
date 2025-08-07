use crate::domain::errors::service::AppErrorInfo;

pub trait ServiceErrorExt {
    fn map_service_error<E: AppErrorInfo>(&self, err: E) -> String {
        tracing::error!("{} | Level: {:?}", err.log_message(), err.level());
        err.client_message()
    }
}
