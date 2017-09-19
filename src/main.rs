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

fn main() {
    cli_clap::cli_clap();
}
