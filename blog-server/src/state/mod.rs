use crate::auth::{UserToken, validate::ValidateData};
use jsonwebtoken::get_current_timestamp;
use rocket::http::Status;
use serde::{Deserialize, Serialize};

pub fn get_default_user_token() -> UserToken {
    UserToken {
        id: uuid::Uuid::new_v4().to_string(),
        expire_time: get_current_timestamp(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct AppState {

}

pub fn init_app_state() -> AppState {
    
    AppState {}
}

pub fn init_validate_instace () -> Result<ValidateData, Status> {
    let instance = ValidateData::new()?;
    Ok(instance)
}
