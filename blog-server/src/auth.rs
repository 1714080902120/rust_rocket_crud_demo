use std::{cmp::Ordering, io::Cursor};
use jsonwebtoken::{decode, get_current_timestamp, DecodingKey, Validation};
use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request,
    Response, http::Status, serde::json::serde_json::json,
};
use serde::{Deserialize, Serialize};
const KEY: &str = "dan";

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct UserToken {
    pub id: i32,
    pub expire_time: u64,
}

pub struct AuthMsg {
    pub is_valid_token: bool,
}

#[rocket::async_trait]
impl Fairing for UserToken {
    fn info(&self) -> Info {
        Info {
            name: "user authorized",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let header = request.headers();
        let token_data = header.get("_token").next();

        let token = match token_data {
            Some(token) => {
                match decode::<UserToken>(
                    token,
                    &DecodingKey::from_secret(KEY.as_ref()),
                    &Validation::new(jsonwebtoken::Algorithm::ES256),
                ) {
                    Ok(user_token) => user_token,
                    Err(err) => {
                        request.local_cache(|| AuthMsg {
                            is_valid_token: false,
                        });
                        dbg!(err);
                        return;
                    }
                }
            }
            None => {
                request.local_cache(|| AuthMsg {
                    is_valid_token: false,
                });
                return;
            }
        };
        // validate time
        let expire_time: u64 = token.claims.expire_time;
        match get_current_timestamp().cmp(&expire_time) {
            Ordering::Less => {
                request.local_cache(|| AuthMsg {
                    is_valid_token: false,
                });
            }
            _ => {
                request.local_cache(|| AuthMsg {
                    is_valid_token: true,
                });
            }
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
      let auth_state = req.local_cache(|| AuthMsg { is_valid_token: false });
      
      if !auth_state.is_valid_token {
        res.set_status(Status::NonAuthoritativeInformation);
        
        let data = json!({
            "success": false,
            "rt": -1,
            "data": (),
            "msg": String::from("user not login or expired token !")
        });

        let data_str = data.to_string().as_str();

        res.set_sized_body(data_str.len(), Cursor::new(data_str));
        
      }

    }
    
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct RtData {
  pub success: bool,
  pub rt: i32,
  pub data: (),
  pub msg: String
}
