use config::{Config, ConfigError, Map, Value};

pub struct Server {
    pub host: String,
    pub port: u16,
}

pub struct Settings {
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = &Config::builder()
            .add_source(config::File::with_name("Configuration"))
            .build()?;
        Ok(Self {
            server: config.get_table("server")?.into()
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
            host: map.remove("host").unwrap()
                .into_string().unwrap(),
            port: u16::try_from(
                map.remove("port").unwrap()
                    .into_uint().unwrap()
            ).unwrap()
        }
    }
}