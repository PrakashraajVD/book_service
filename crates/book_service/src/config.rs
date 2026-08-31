use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct AppConf {
    #[envconfig(nested)]
    pub server: ServerConf,
}

impl AppConf {
    pub fn init() -> Self {
        Self::init_from_env().expect("Failed to load configuration file! Check the .env file")
    }
}

#[derive(Clone, Envconfig)]
pub struct ServerConf {
    #[envconfig(from = "SERVER_PORT", default = "3000")]
    pub port: u16,
    #[envconfig(from = "SERVER_ALLOWED_ORIGINS")]
    pub allowed_origins: String,
    #[envconfig(from = "SERVER_ALLOWED_METHODS")]
    pub allowed_methods: String,
    #[envconfig(from = "SERVER_ALLOWED_HEADERS")]
    pub allowed_headers: String,
    #[envconfig(from = "SERVER_DEFAULT_BODY_LIMIT", default = "1048576")]
    pub default_body_limit: usize,
}

impl ServerConf {
    pub fn to_addr(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
