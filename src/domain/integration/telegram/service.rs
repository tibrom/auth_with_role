use super::model::TelegramData;

pub trait ParserInitDataService{
    fn get_tg_id(&self) -> Option<i64>;
    fn get_tg_username(&self) -> Option<String>;
    fn first_name(&self) -> Option<String>;
    fn last_name(&self) -> Option<String>;
}

pub trait FactoryTelegramInitDataParser{
    type Service: ParserInitDataService;
    fn create(&self, init_data: String) -> Self::Service;
}