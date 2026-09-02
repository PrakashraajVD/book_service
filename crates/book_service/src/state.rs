use toasty::Db;

use crate::config::ServerConf;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub server_conf: ServerConf,
}
