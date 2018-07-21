use std::collections::HashSet;
use std::result::Result;
use std::vec::Vec;

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub enum Action {
    DbConnect,
    ListProvides,
    LoadJobs,
    ListTarget,
    LoadScripts,
    SessionStart,
    RequestRun,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Config {
    pub actions: HashSet<Action>,
    pub rdbms_connection_uri: Option<String>,
    pub enviroment: Vec<String>,
    pub path_jobs: Vec<String>,
    pub path_shell: Vec<String>,
    pub path_python: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        Ok(Config {
            actions: HashSet::new(),
            rdbms_connection_uri: None,
            enviroment: Vec::new(),
            path_jobs: Vec::new(),
            path_shell: Vec::new(),
            path_python: Vec::new(),
        })
    }
}
