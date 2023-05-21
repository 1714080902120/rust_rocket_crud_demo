use jsonwebtoken::{get_current_timestamp};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{ContentType, Status},
    Data, Request, Response,
};
use std::{cmp::Ordering, io::Cursor};

use crate::{types::{RtData, FailureData}, config::MyConfig};
use crate::auth::{ UserToken, AuthMsg};

use super::token::decode_token;



const EXCEPT_LIST: [Status; 3] = [Status::NotFound, Status::InternalServerError, Status::BadRequest];

#[rocket::async_trait]
impl Fairing for UserToken {
    fn info(&self) -> Info {
        Info {
            name: "user authorized",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        
        let my_config = request.rocket().state::<MyConfig>().expect("get global custom config error in fairing");
        let token_field = my_config.token_field.as_str();
        let token_key = my_config.token_key.as_str();

        let header = request.headers();
        let token_data = header.get(token_field).next();
        
        let token = match token_data {
            Some(token) => {
                dbg!(token);
                match decode_token(token, token_key) {
                    Ok(user_token) => user_token,
                    Err(err) => {
                        dbg!(err);
                        return;
                    }
                }
            }
            None => {
                dbg!("has none token");
                return;
            }
        };
        // validate time
        let exp: u64 = token.claims.exp;
        match get_current_timestamp().cmp(&exp) {
            Ordering::Less => {
                request.local_cache(|| AuthMsg {
                    is_valid_token: true,
                });
            }
            _ => {}
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let auth_state = req.local_cache(|| AuthMsg {
            is_valid_token: false,
        });


        dbg!(auth_state);

        if !auth_state.is_valid_token && !EXCEPT_LIST.contains(&res.status()) {
            res.set_status(Status::NonAuthoritativeInformation);
            res.set_header(ContentType::JSON);
            let data = RtData {
                success: false,
                rt: -1,
                data: FailureData(()),
                msg: String::from("user not login or expired token !")
            };
            

            let data_str = data.to_string();

            res.set_sized_body(data_str.len(), Cursor::new(data_str));
        }
    }
}
