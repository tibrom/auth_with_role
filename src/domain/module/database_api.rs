use crate::http::hasura::{HasuraClient, HasuraClientBuilder};

use include_dir::{include_dir, Dir};
use serde_json::{json, Value};
use super::user::ModuleUser;





#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct HasurasModuleUser {
    pub user: Vec<ModuleUser>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct HasurasVisitorScheduleByPhone {
    pub scheduling_visitor_schedule_by_phone: Vec<VisitorSchedule>,
}



#[derive(Clone, Debug)]
pub struct UserApi {
    gql_client: HasuraClient,
}

impl UserApi {
    //noinspection DuplicatedCode
    pub fn new(host: String, api_key: Option<String>) -> Self {
        Self {
            gql_client: HasuraClientBuilder::new(host, FEAT_DIR_NAME)
                .add_filename(GET_USER_BY_EMAIL)
                .add_filename(GET_USER_BY_ID)
                .add_filename(GET_USER_BY_TG_ID)
                .build()
        }
    }
}




impl UserApi {
    pub async fn query_scheduling_visitor_schedule_by_phone(&self, phone: &str) -> Result<Vec<VisitorSchedule>, String> {
        const ERR_MSG: &str = "Невозможно получить график, причина:";
        let variables = Some(json!({ "phone_number": phone }));
        let value = self.gql_client
            .execute(SCHEDULING_VISITOR_SCHEDULE_BY_PHONE, variables)
            .await
            .map_err(|e| format!("{ERR_MSG} '{e:?}'"))?;


        let result: HasurasVisitorScheduleByPhone = serde_json::from_value(value)
            .map_err(|e| format!("{ERR_MSG} ошибка обработки ответа: {e:?}"))?;

        Ok(result.scheduling_visitor_schedule_by_phone)

    }

    pub async fn query_scheduling_visitor_schedule_by_q_number(&self, code_number: &str) -> Result<Vec<VisitorSchedule>, String> {
        const ERR_MSG: &str = "Невозможно получить график, причина:";
        let variables = Some(json!({ "code_number": code_number }));
        let value = self.gql_client
            .execute(SCHEDULING_VISITOR_SCHEDULE_BY_Q_NUMBER, variables)
            .await
            .map_err(|e| format!("{ERR_MSG} '{e:?}'"))?;
        println!("Ответ {:?}", value);
        let result: HasurasVisitorScheduleByQNumber = serde_json::from_value(value)
            .map_err(|e| format!("{ERR_MSG} ошибка обработки ответа: {e:?}"))?;
        println!("Результат {:?}", result);

        Ok(result.scheduling_visitor_schedule_by_q_number)
    }

    pub async fn mut_ft_reg_kiosk_f_create_visitor_card_by_schedule_id(&self, schedule_id: &u64) -> Result<VisitorCard, String> {
        const ERR_MSG: &str = "Невозможно получить график, причина:";
        let variables = Some(json!({ "id": schedule_id }));
        let value = self.gql_client
            .execute(FT_REG_KIOSK_F_CREATE_VISITOR_CARD_BY_SCHEDULE_ID, variables)
            .await
            .map_err(|e| format!("{ERR_MSG} '{e:?}'"))?;

        let result: HasurasSystemApi = serde_json::from_value(value)
            .map_err(|e| format!("{ERR_MSG} ошибка обработки ответа: {e:?}"))?;

        Ok(result.visitor_card)
    }

    pub async fn mut_ft_reg_kiosk_f_create_visitor_card_for_service(&self, service_uuid: &str, weight: u64) ->  Result<VisitorCard, String> {
        tracing::trace!("mut_ft_reg_kiosk_f_create_visitor_card_for_service");
        const ERR_MSG: &str = "Невозможно получить график, причина:";
        let variables = Some(json!({ "service_uuid": service_uuid, "weight": weight }));
        let value = self.gql_client
            .execute(FT_REG_KIOSK_F_CREATE_VISITOR_CARD_FOR_SERVICE, variables)
            .await
            .map_err(|e| format!("{ERR_MSG} '{e:?}'"))?;

        let result: HasurasSystemApi = serde_json::from_value(value)
            .map_err(|e| format!("{ERR_MSG} ошибка обработки ответа: {e:?}"))?;

        Ok(result.visitor_card)
    }

    

}