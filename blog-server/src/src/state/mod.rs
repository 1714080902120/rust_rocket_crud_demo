use crate::auth::{UserToken, validate::ValidateData};
use jsonwebtoken::get_current_timestamp;
use rocket::http::Status;

pub fn get_default_user_token() -> UserToken {
    UserToken {
        id: uuid::Uuid::new_v4().to_string(),
        exp: get_current_timestamp(),
    }
}

pub fn init_validate_instace () -> Result<ValidateData, Status> {
    let instance = ValidateData::new()?;
    Ok(instance)
}
