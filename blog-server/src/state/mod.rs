use crate::auth::UserToken;
use jsonwebtoken::get_current_timestamp;
use serde::{Deserialize, Serialize};

pub fn get_default_user_token() -> UserToken {
    UserToken {
        id: -1,
        expire_time: get_current_timestamp(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct AppState {}

pub fn init_app_state() -> AppState {
    
    AppState {}
}
