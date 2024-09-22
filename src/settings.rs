use config::{Config, ConfigError, Map, Value};

#[derive(Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Clone)]
pub struct Jwt {
    pub secret: String,
    pub validity_days: u16
}

#[derive(Clone)]
pub struct Settings {
    pub server: Server,
    pub jwt: Jwt,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = &Config::builder()
            .add_source(config::File::with_name("Configuration"))
            .build()?;
        Ok(Self {
            server: config.get_table("server")?.into(),
            jwt: config.get_table("jwt")?.into(),
        })
    }
}

impl Server {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl From<Map<String, Value>> for Server {
    fn from(mut map: Map<String, Value>) -> Self {
        Server {
            host: map.remove("host").expect("Server host must be set")
                .into_string().unwrap(),
            port: map.remove("port").expect("Server port must be set")
                .into_uint().unwrap() as u16,
        }
    }
}

impl From<Map<String, Value>> for Jwt {
    fn from(mut map: Map<String, Value>) -> Self {
        Jwt {
            secret: map.remove("secret")
                .expect("JWT secret must be set")
                .into_string().unwrap(),
            validity_days: map.remove("validity_days")
                .expect("JWT secret must be set")
                .into_uint().unwrap() as u16
        }
    }
}