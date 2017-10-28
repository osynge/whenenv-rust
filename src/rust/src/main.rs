extern crate clap;
extern crate rusqlite;
#[macro_use]

extern crate rustc_serialize;
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

use uuid::Uuid;


fn main() {
    let session_uuid = Uuid::new_v4();
    let session_uuid_string = session_uuid.simple().to_string();
    let conn = db::connect();
    db::create_tables(&conn);
    let some_value = 10;
    let clap_matches = cli_clap::cli_clap(&some_value);
    loader::deligate(&conn, &clap_matches);
    let pk_session = elephant::elephant_session(&conn, &session_uuid_string);
    loader::enviroment(&conn, pk_session, &clap_matches);
    jobs_load::load(&conn);
}
