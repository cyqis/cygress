mod module;

use serde::Deserialize;
use serde_yaml;
use std::env;
use std::fs::File;

use module::*;

const CONFG_PATH: &str = "cygress.yaml";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cfg {
    pub listener: Listener,
    pub backend: Backend,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            listener: Listener::default(),
            backend: Backend::default(),
        }
    }
}

impl Cfg {
    pub fn load_from_env() -> Self {
        let path = env::var("CYGRESS_CONFG_PATH").unwrap_or(CONFG_PATH.to_string());
        let file = File::open(path).unwrap();
        let sconf: Cfg = serde_yaml::from_reader(file).unwrap();
        sconf
    }
}
