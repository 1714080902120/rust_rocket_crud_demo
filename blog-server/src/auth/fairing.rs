use jsonwebtoken::{decode, get_current_timestamp, DecodingKey, Validation};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{ContentType, Status},
    Data, Request, Response,
};
use std::{cmp::Ordering, io::Cursor};

use crate::types::{RtData, FailureData};
use crate::auth::{ UserToken, AuthMsg};


const KEY: &str = "dan";

const EXCEPT_LIST: [Status; 2] = [Status::NotFound, Status::InternalServerError];

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
        let auth_state = req.local_cache(|| AuthMsg {
            is_valid_token: false,
        });

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
