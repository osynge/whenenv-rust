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
    db::create_tables(&conn);


    let clp_matches = cli_clap::cli_clap(&conn);
    loader::deligate(&conn, clp_matches);

    jobs_load::load(&conn);
}
