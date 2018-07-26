use actions_process_list_provides;
use cfg;
use db;
use rusqlite::Connection;
use std::sync::Arc;
use std::sync::Mutex;

fn process_session(conn: &Connection) {}

pub fn process(conn: &Connection, rt_cfg_arc_mu: &Arc<Mutex<cfg::Config>>) {
    let rt_cfg = rt_cfg_arc_mu.lock().unwrap();
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

pub fn cfg_process_action_db_connect(rt_cfg_arc_mu: &Arc<Mutex<cfg::Config>>) -> Connection {
    let mut fred = rt_cfg_arc_mu.lock().unwrap();
    match fred.rdbms_connection_uri {
        Some(ref connection_uri) => {
            return db::connect_file(&connection_uri);
        }
        None => {
            return db::connect();
        }
    }
}
