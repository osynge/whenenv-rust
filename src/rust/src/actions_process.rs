use rusqlite::Connection;
use std::collections::HashSet;
use actions_process_list_provides;

fn process_session(conn: &Connection) {}

pub fn process(conn: &Connection, actions: &HashSet<String>) {
    let matcher_session = String::from("session");
    if actions.contains(&matcher_session) {
        process_session(conn);
    }
    let matcher_list_provides = String::from("list-provides");
    if actions.contains(&matcher_list_provides) {
        actions_process_list_provides::process_list_provides(conn);
    }
    let matcher_list_target = String::from("list-target");
    if actions.contains(&matcher_list_target) {
        actions_process_list_provides::process_list_targets(conn);
    }
}
