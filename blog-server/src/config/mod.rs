use rocket::{
    fairing::AdHoc,
    figment::{
        providers::{Format, Serialized, Toml},
        Figment,
    },
    Config,
};
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct MyConfig {
    pub token_field: String,
    pub token_key: String,
    pub exp: u64,
}

impl Default for MyConfig {
    fn default() -> Self {
        Self {
            token_field: String::from("_token"),
            token_key: String::from("dan"),
            exp: 24 * 60 * 60 * 1000, // one day
        }
    }
}

pub fn get_custom_figment() -> Figment {
    Figment::from(Config::default())
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Serialized::defaults(MyConfig::default()))
}

pub fn init_my_config() -> AdHoc {
    AdHoc::config::<MyConfig>()
}
