use rocket::{Config, figment::{Figment, providers::{Toml, Format}}};

pub fn get_custom_figment() -> Figment {
    let email_reg_rule =
        r"^[a-z0-9A-Z]+[- | a-z0-9A-Z . _]+@([a-z0-9A-Z]+(-[a-z0-9A-Z]+)?\\.)+[a-z]{2,}$";
    let phone_reg_rule = r"/^1(3\d|4[5-9]|5[0-35-9]|6[567]|7[0-8]|8\d|9[0-35-9])\d{8}$/";

    Config::figment()
        .merge(Toml::file("Rocket.toml").nested())
        .merge(("email_reg_rule", email_reg_rule))
        .merge(("phone_reg_rule", phone_reg_rule))
}
