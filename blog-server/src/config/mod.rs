use rocket::{
    figment::{
        providers::{Format, Serialized, Toml},
        Figment,
    },
    Config, fairing::AdHoc,
};
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct MyConfig {
    pub email_reg_rule: String,
    pub phone_reg_rule: String,
    pub token_field: String,
    pub token_key: String,
    pub expire_time: u64,
}

impl Default for MyConfig {
    fn default() -> Self {
        Self {
            email_reg_rule:
                r"^[a-zA-Z0-9_-]+@[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)+$"
                    .to_string(),
            phone_reg_rule: r"^1(3\d|4[5-9]|5[0-35-9]|6[567]|7[0-8]|8\d|9[0-35-9])\d{8}$"
                .to_string(),
                token_field: String::from("_token"),
                token_key: String::from("dan"),
                expire_time: 24 * 60 * 60 * 1000  // one day
        }
    }
}

pub fn get_custom_figment() -> Figment {
    Figment::from(Config::default())
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Serialized::defaults(MyConfig::default()))
}

pub fn init_my_config () -> AdHoc {
    AdHoc::config::<MyConfig>()
}
