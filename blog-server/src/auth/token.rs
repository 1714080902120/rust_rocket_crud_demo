use jsonwebtoken::{
    decode, encode, errors::Error, get_current_timestamp, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};

use rocket::Request;

use super::{UserToken};

use crate::config::MyConfig;



pub fn decode_token(token: &str, key: &str) -> Result<TokenData<UserToken>, Error> {
    
    decode::<UserToken>(
        token,
        &DecodingKey::from_secret(key.as_ref()),
        &Validation::default(),
    )
}

pub fn encode_token(user_id: String, exp: u64, key: &str) -> String {
    let user_token = UserToken {
        id: user_id,
        exp,
    };
    encode::<UserToken>(
        &Header::default(),
        &user_token,
        &EncodingKey::from_secret(key.as_ref()),
    )
    .expect("encode token error")
}

pub fn set_token<'r>(req: &'r Request<'_>, user_id: &str) -> (String, String) {
    let my_config = req
        .rocket()
        .state::<MyConfig>()
        .expect("get global state error when response in login");
    let token_field = my_config.token_field.as_str();
    let token_key = my_config.token_key.as_str();
    let exp = my_config.exp + get_current_timestamp();

    let token = encode_token(user_id.to_string(), exp, token_key);

    (token_field.to_string(), token)
}
