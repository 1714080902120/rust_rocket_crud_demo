use jsonwebtoken::{
    decode, encode, get_current_timestamp, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
    errors::Error
};

use super::UserToken;

pub fn decode_token(token: &str, key: &str) -> Result<TokenData<UserToken>, Error> {
    decode::<UserToken>(
        token,
        &DecodingKey::from_secret(key.as_ref()),
        &Validation::default(),
    )
}

pub fn encode_token(user_id: String, expired_time: u64, key: &str) -> String {
    let user_token = UserToken {
        id: user_id,
        expire_time: get_current_timestamp() + expired_time,
    };
    encode::<UserToken>(
        &Header::default(),
        &user_token,
        &EncodingKey::from_secret(key.as_ref()),
    )
    .expect("encode token error")
}
