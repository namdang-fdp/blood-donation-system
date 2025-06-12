pub mod bcrypt;
pub mod jwt;
pub mod oidc;

use std::{collections::HashMap, sync::LazyLock};

use oidc::Provider;
use serde::Deserialize;

use crate::config::{bcrypt::BcryptConfig, jwt::JwtConfig, oidc::OpenIdConnectConfig};

const fn default_port() -> u16 {
    3000
}

fn default_frontend_url() -> String {
    "http://localhost:3001".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_frontend_url")]
    pub frontend_url: String,

    #[serde(default)]
    pub bcrypt: BcryptConfig,
    #[serde(default)]
    pub jwt: JwtConfig,
    pub oidc: HashMap<Provider, OpenIdConnectConfig>,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    ::config::Config::builder()
        .add_source(::config::File::with_name("config").required(false))
        .add_source(
            ::config::Environment::default()
                .try_parsing(true)
                .separator("__"),
        )
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
});
