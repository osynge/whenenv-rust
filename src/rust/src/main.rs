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
mod cfg;
mod clap_actions;
mod clap_fern;
mod cli_clap;
mod db;
use db::db_enviroment;
use db::db_fs_dir;
use db::db_fs_file;
use db::db_job;
use db::db_job_depend;
use db::db_job_provide;
use db::db_job_require_variable;
use db::db_job_require_variable_pair;
use db::db_provider;
use db::db_session;
use db::fs_dir_type;
mod db_variable_name;
mod db_variable_pair;
mod elephant;
mod jobs_load;
mod json_loader_elephant;
mod loader;
use uuid::Uuid;

fn main() {
    let mut runtime_cfg = cfg::Config::new().unwrap();
    let clap_matches = cli_clap::cli_clap();
    clap_fern::log_setup(&clap_matches);
    clap_actions::cfg_actions_update_clap(&mut runtime_cfg, &clap_matches);
    let session_uuid = Uuid::new_v4();
    let session_uuid_string = session_uuid.hyphenated().to_string();
    trace!("session_uuid_string:{}", session_uuid_string);
    let conn = actions_process::cfg_process_action_db_connect(&mut runtime_cfg);
    db::create_tables(&conn);
    loader::deligate(&conn, &mut runtime_cfg);
    let pk_session = elephant::elephant_session(&conn, &session_uuid_string);
    loader::enviroment(&conn, &mut runtime_cfg, pk_session);
    jobs_load::load(&conn);
    actions_process::process(&conn, &runtime_cfg)
}
