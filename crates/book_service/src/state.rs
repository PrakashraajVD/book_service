use crate::config::ServerConf;

#[derive(Clone)]
pub struct AppState {
    pub server_conf: ServerConf,
}
