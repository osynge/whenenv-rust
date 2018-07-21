use actions_process_list_provides;
use cfg;
use db;
use rusqlite::Connection;

fn process_session(conn: &Connection) {}

pub fn process(conn: &Connection, rt_cfg: &cfg::Config) {
    if rt_cfg.actions.contains(&cfg::Action::SessionStart) {
        process_session(conn);
    }
    if rt_cfg.actions.contains(&cfg::Action::ListProvides) {
        actions_process_list_provides::process_list_provides(conn);
    }
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
