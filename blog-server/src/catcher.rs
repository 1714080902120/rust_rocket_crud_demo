use std::path::Path;
use crate::types::{ FailureData, RtData };

use rocket::{
    serde::json::{serde_json::json, Value}, fs::NamedFile,
    catch,
};

#[catch(400)]
pub fn bad_request_catcher() -> Option<RtData<FailureData>> {
    Some(RtData {
        success: false,
        rt: -3,
        msg: String::from("get wrong params"),
        data: FailureData(())
    })
}


#[catch(404)]
pub async fn not_found_catcher() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/404.html")).await.ok()
}

#[catch(500)]
pub fn error_catcher() -> Option<Value> {
    Some(json!({
            "success": false,
            "rt": -2,
            "msg": String::from("server internal error"),
            "data": FailureData(()),
    }))
}
