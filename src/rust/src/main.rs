extern crate clap;
extern crate rusqlite;
#[macro_use]
extern crate log;
extern crate fern;

extern crate chrono;
extern crate rustc_serialize;
extern crate serde_yaml;
extern crate uuid;

mod loader;
mod db;
mod dbFsDirType;
mod dbFsDir;
mod dbFsFile;
mod dbProvider;
mod dbJob;
mod dbJobProvide;
mod dbJobDepend;
mod dbVariableName;
mod dbJobRequireVariable;
mod dbVariablePair;
mod dbJobRequireVariablePair;
mod json_loader_elephant;
mod cli_clap;
mod jobs_load;
mod elephant;
mod dbSession;
mod dbEnviroment;
mod autoconf;
mod clap_actions;
mod actions_process;
mod actions_process_list_provides;
mod clap_fern;

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
