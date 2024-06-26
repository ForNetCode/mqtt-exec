use std::env;
use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

pub const APP_NAME: &str = "mproxy";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    command_topic: Option<String>,
    response_command_topic: Option<String>,
    pub server: String,
    pub client_id: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Config {
    pub fn new(config_path: Option<PathBuf>) -> anyhow::Result<Self> {
        let config_path = if let Some(config_path)= config_path {
            config_path
        } else {
            Self::get_default_config_path()?
        };
        if !config_path.is_file() {
            bail!("config file not found: {:?}", config_path);
        }
        info!("load config from {:?}", config_path);
        let config = std::fs::read_to_string(config_path)?;
        let config: Config = serde_yml::from_str(&config)?;
        Ok(config)
    }

    fn get_default_config_path() -> anyhow::Result<PathBuf> {
        let path = get_exec_path()?;
        Ok(path.join(format!("{APP_NAME}.yml")))
    }


    pub fn get_command_topic(&self) -> String {
        self.command_topic.clone().unwrap_or_else(||format!("cmd/{}", self.client_id))

    }
    pub fn get_response_command_topic(&self) ->  String {
        self.command_topic.clone().unwrap_or_else(||format!("cmd/{}/resp",  self.client_id))
    }
}

pub const CTRL_APP_NAME: &str = "mpublish";


#[derive(Serialize, Deserialize, Debug)]
pub struct CtrlConfig {
    pub server: String,
    pub client_id: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub subscribe_client_id: String,
    publish_command_topic: Option<String>,
    subscribe_command_topic: Option<String>,
}
impl CtrlConfig {
    pub fn new(config_path:Option<PathBuf>) -> anyhow::Result<Self> {
        let config_path = if let Some(config_path)= config_path {
            config_path
        } else {
            Self::get_default_config_path()?
        };
        if !config_path.is_file() {
            bail!("config file not found: {:?}", config_path);
        }
        info!("load ctrl config from {:?}", config_path);
        let config = std::fs::read_to_string(config_path)?;
        let config: Self = serde_yml::from_str(&config)?;
        Ok(config)
    }

    fn get_default_config_path() -> anyhow::Result<PathBuf> {
        let path = get_exec_path()?;
        Ok(path.join(format!("{CTRL_APP_NAME}.yml")))
    }

    pub fn get_publish_command_topic(&self) -> String {
        self.publish_command_topic.clone().unwrap_or_else(||format!("cmd/{}", self.subscribe_client_id))

    }
    pub fn get_subscribe_command_topic(&self) ->  String {
        self.subscribe_command_topic.clone().unwrap_or_else(||format!("cmd/{}/resp",  self.subscribe_client_id))
    }
}



fn get_exec_path() -> anyhow::Result<PathBuf>{
    let path = env::current_exe().with_context(||"can not get exe path")?;
    if let Some(path) = path.parent() {
        return Ok(path.to_path_buf())
    }
    bail!("can not get current exe path");
}

#[cfg(test)]
mod test {
    use crate::config::Config;

    #[test]
    pub fn get_default_config_path_test() {
        println!("{:?}", Config::get_default_config_path());
    }

    #[test]
    pub fn test_config_serial() {
        let config = Config::new(Some("./config.yml".parse().unwrap())).unwrap();
        println!("{:?}", config)
    }
}
