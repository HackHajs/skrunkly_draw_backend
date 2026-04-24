use std::{fs, net::SocketAddr, path::PathBuf, sync::LazyLock};

use axum::http::HeaderValue;
use jsonwebtoken::jwk::JwkSet;
use serde::Deserialize;

pub mod headervalues {
    use axum::http::HeaderValue;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[allow(clippy::missing_errors_doc)]
    pub fn deserialize<'de, D>(d: D) -> Result<Vec<HeaderValue>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let strs: Vec<String> = Vec::deserialize(d)?;
        strs.into_iter()
            .map(|s| HeaderValue::from_str(&s).map_err(serde::de::Error::custom))
            .collect()
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn serialize<S>(v: &[HeaderValue], s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let strs: Vec<&str> = v.iter().map(|h| h.to_str().unwrap_or_default()).collect();
        strs.serialize(s)
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub server: Server,
    pub database: Database,
    pub auth: Auth,
}

#[derive(Deserialize)]
pub struct Server {
    pub bind_address: SocketAddr,
    pub cors: Cors,
}

#[derive(Deserialize)]
pub struct Cors {
    #[serde(with = "headervalues")]
    pub allowed_origins: Vec<HeaderValue>,
}

#[derive(Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Deserialize)]
pub struct Auth {
    pub jwk: Jwk,
}

#[derive(Deserialize)]
pub struct Jwk {
    pub issuers: Vec<String>,
    pub set: JwkSet,
    pub authenticated_audiences: Vec<String>,
    pub admin_role: String,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let config_path = std::env::args().nth(1).map_or_else(
        || {
            log::info!(
                "Config path not provided, using default: /etc/skrunkly_draw_backend/config.toml"
            );
            PathBuf::from("/etc/skrunkly_draw_backend/config.toml")
        },
        PathBuf::from,
    );

    let contents = match fs::read_to_string(&config_path) {
        Ok(contents) => contents,
        Err(err) => {
            log::error!("Could not read the config file at {config_path:?}: {err}");
            panic!();
        }
    };

    match toml::from_str(&contents) {
        Ok(configuration) => configuration,
        Err(err) => {
            log::error!("Could not parse config: {err}");
            panic!();
        }
    }
});
