use actions_process_list_provides;
use cfg;
use clap::ArgMatches;
use db;
use rusqlite::Connection;
use std::collections::HashSet;

fn process_session(conn: &Connection) {}

pub fn process(conn: &Connection, rt_cfg: &cfg::Config) {
    let matcher_session = String::from("session");
    if rt_cfg.actions.contains(&cfg::Action::SessionStart) {
        process_session(conn);
    }
    let matcher_list_provides = String::from("list-provides");
    if rt_cfg.actions.contains(&cfg::Action::ListProvides) {
        actions_process_list_provides::process_list_provides(conn);
    }
    let matcher_list_target = String::from("list-target");
    if rt_cfg.actions.contains(&cfg::Action::ListTarget) {
        actions_process_list_provides::process_list_targets(conn);
    }
}

pub fn cfg_process_action_db_connect(fred: &mut cfg::Config) -> Connection {
    match fred.rdbms_connection_uri {
        Some(ref connection_uri) => {
            return db::connect_file(&connection_uri);
        }
        None => {
            return db::connect();
        }
    }
}
