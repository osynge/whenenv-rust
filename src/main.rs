extern crate clap;
extern crate rusqlite;
#[macro_use]

extern crate rustc_serialize;



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


fn main() {
    let conn = db::connect();
    let session_uuid = "".to_string();
    db::create_tables(&conn);
    
    let some_value = 10;
    let clap_matches = cli_clap::cli_clap(&some_value);
    loader::deligate(&conn, &clap_matches);
    let pk_session = elephant::elephant_session(&conn, &session_uuid);
    loader::enviroment(&conn, pk_session, &clap_matches);
    jobs_load::load(&conn);
}
