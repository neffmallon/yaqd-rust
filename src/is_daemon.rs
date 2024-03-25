use dirs;
use serde::{Deserialize, Serialize};
use toml;
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub struct DaemonID {
    name: String,
    kind: String,
    make: Option<String>,
    model: Option<String>,
    serial: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LogLevel{
    INFO,
    NOTICE,
    WARNING,
    ERROR,
    CRITICAL,
    ALERT,
    EMERGENCY,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DaemonConfig {
    port: u16,
    serial: Option<String>,
    make: Option<String>,
    model: Option<String>,
    enable: bool,
    log_level: LogLevel,
    log_to_file: bool,
}

impl Default for DaemonConfig {
    fn default() -> DaemonConfig {
        DaemonConfig {
            port: 0,
            serial: None,
            make: None,
            model: None,
            enable: true,
            log_level: LogLevel::INFO,
            log_to_file: false
        }
    }
}


#[derive(Deserialize, Serialize)]
pub struct BaseDaemonState{
    busy: bool,
}


/// IsDaemon defines the minimum required function calls that need to be implemented by every
pub trait IsDaemon{

    /// Gets the state in a type-safe way to be used for other methods
    fn _get_state(&self) -> &BaseDaemonState;

    /// Gets the state in a type-safe way to be used for other methods
    fn _get_config(&self) -> &DaemonConfig;

    /// Gets the logger for the object for other methods
    fn _get_logger(&self);

    /// Gets the DaemonID object from the Daemon
    fn _id(&self) -> &DaemonID;

    /// Gets the DaemonID in JSON form!
    fn id(&self) -> String{
        serde_json::to_string(self._id())?
    }

    // Boolean representing if the daemon is busy (state updated) or not.
    fn busy(&self) -> bool{
        self._get_state().busy
    }

    /// Retrieve the current filepath of the configuration.
    fn get_config_filepath(&self) -> String {
        dirs::data_local_dir().unwrap()
            .join("yaqd")
            .join(&self._id().kind)
            .join("config.toml")
            .to_str().unwrap().to_string()
    }

    /// Retrieve the current configuration, including any defaults.
    fn get_config(&self) -> String{
        toml::to_string(&self._get_config()).unwrap()
    }

    fn get_state_filepath(&self) -> String {
        dirs::data_local_dir().unwrap()
            .join("yaqd-state")
            .join(&self._id().kind)
            .join(format!("{}-state.toml",self._id().name))
            .to_str().unwrap().to_string()
    }

    ///Get version of the running daemon
    fn get_state(&self) -> String{
        toml::to_string(&self._get_state()).unwrap()
    }

    /// Cleanly shutdown (or restart) daemon
    fn shutdown(&self, restart: Option<bool>);

}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;
    use toml;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Config {
        ip: String,
        port: Option<u16>,
        keys: Keys,
    }

    #[derive(Deserialize)]
    struct Keys {
        github: String,
        travis: Option<String>,
    }

    #[test]
    fn test_table() {
        let value = read_to_string(r"/home/nathan/PycharmProjects/yaq-rust/newport-conex-agp.toml")
            .unwrap();
        println!("{:?}", value.parse::<toml::Table>().unwrap()["config"])
    }

    #[test]
    fn mess_around() {
        let mut config: Config = toml::from_str(r#"
ip = '127.0.0.1'

[keys]
github = 'xxxxxxxxxxxxxxxxx'
travis = 'yyyyyyyyyyyyyyyyy'
"#).unwrap();

        assert_eq!(config.ip, "127.0.0.1");
        assert_eq!(config.port, None);
        assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
        assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
        config.keys.travis = Some("Hello, world!".to_string());
        assert_eq!(config.keys.travis.as_ref().unwrap(), "Hello, world!");
        println!("It go okay! (0)")

    }

    #[test]
    fn mess_around_1() {
        let config: Config = toml::from_str(r#"
ip = '127.0.0.1'

[keys]
github = 'xxxxxxxxxxxxxxxxx'
travis = 'yyyyyyyyyyyyyyyyy'
bananas = 'qwerqwerqwerqwer'
"#).unwrap();

        assert_eq!(config.ip, "127.0.0.1");
        assert_eq!(config.port, None);
        assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
        assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
        println!("It go okay! (1)")
    }
}