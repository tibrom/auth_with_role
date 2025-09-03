use crate::domain::errors::service::AppErrorInfo;
use super::model::TelegramData;
pub trait TelegramVerifierService {
    type Error: AppErrorInfo;
    fn is_verified_telegram_data(&self, telegram_data: TelegramData) -> Result<bool, Self::Error>;
    fn is_verified_mini_app_data(&self, init_data: &str) -> Result<bool, Self::Error>;
}