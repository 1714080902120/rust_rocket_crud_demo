use rocket::{
    post, Request
};

use crate::auth::LoginData;



#[post("/login", data = "<login_data>")]
pub fn login(login_data: LoginData) {
    // 这里就简单的校验下即可
    
}