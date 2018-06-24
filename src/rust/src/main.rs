extern crate clap;
extern crate rusqlite;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate fern;
extern crate rustc_serialize;
extern crate serde_yaml;
extern crate uuid;
#[cfg(test)]
#[macro_use]
extern crate proptest;

mod actions_process;
mod actions_process_list_provides;
mod autoconf;
mod clap_actions;
mod clap_fern;
mod cli_clap;
mod db;
mod db_enviroment;
mod db_fs_dir;
mod db_fs_dir_type;
mod db_fs_file;
mod db_job;
mod db_job_depend;
mod db_job_provide;
mod db_job_require_variable;
mod db_job_require_variable_pair;
mod db_provider;
use db::db_session;
mod db_variable_name;
mod db_variable_pair;
mod elephant;
mod jobs_load;
mod json_loader_elephant;
mod loader;
use uuid::Uuid;

fn main() {
    let some_value = 10;
    let clap_matches = cli_clap::cli_clap(&some_value);
    let actions = clap_actions::actions_get(&clap_matches);
    clap_fern::log_setup(&clap_matches);
    let session_uuid = Uuid::new_v4();
    let session_uuid_string = session_uuid.hyphenated().to_string();
    trace!("session_uuid_string:{}", session_uuid_string);
    let conn = db::connect_deligate(&clap_matches);
    db::create_tables(&conn);
    loader::deligate(&conn, &actions, &clap_matches);
    let pk_session = elephant::elephant_session(&conn, &session_uuid_string);
    loader::enviroment(&conn, pk_session, &clap_matches);
    jobs_load::load(&conn);
    actions_process::process(&conn, &actions)
}
