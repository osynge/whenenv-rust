extern crate clap;
extern crate rusqlite;
#[macro_use]

extern crate rustc_serialize;


use clap::{App, Arg};

mod loader;
mod db;
mod dbFsDirType;
mod dbFsDir;
mod dbFsFile;
mod dbProvider;
mod dbJob;
mod dbJobProvide;
mod dbJobDepend;

fn main() {

    // Once all App settings (including all arguments) have been set, you call get_matches() which
    // parses the string provided by the user, and returns all the valid matches to the ones you
    // specified.
    //
    // You can then query the matches struct to get information about how the user ran the program
    // at startup.
    //
    // For this example, let's assume you created an App which accepts three arguments (plus two
    // generated by clap), a flag to display debugging information triggered with "-d" or
    // "--debug" as well as an option argument which specifies a custom configuration file to use
    // triggered with "-c file" or "--config file" or "--config=file" and finally a positional
    // argument which is the input file we want to work with, this will be the only required
    // argument.
    let matches = App::new("whenenv")
                        .about("Parses an input file to do awesome things")
                        .version("1.0")
                        .author("Kevin K. <kbknapp@gmail.com>")
                        .arg(Arg::with_name("verbose")
                                    .help("Increase log output.")
                                    .short("v")
                                    .multiple(true) 
                                    .long("verbose"))
                        .arg(Arg::with_name("quiet")
                                    .help("Decrease log output.")
                                    .short("q")
                                    .multiple(true) 
                                    .long("quiet"))
                        .arg(Arg::with_name("env")
                                    .short("e")
                                    .long("env")
                                    .value_name("ENVIROMENT_VARIABLE")
                                    .help("Which enviroment variables to process")
                                    .multiple(true)
                                    .takes_value(true))
                        .arg(Arg::with_name("dir-jobs")
                                    .long("dir-jobs")
                                    .value_name("DIR_JOB")
                                    .help("directory storing json jobs.")
                                    .multiple(true)
                                    .takes_value(true))
                        .arg(Arg::with_name("dir-sh")
                                    .long("dir-sh")
                                    .value_name("DIR_SHELL")
                                    .help("directory storing jobs shell scripts.")
                                    .multiple(true)
                                    .takes_value(true))
                        .arg(Arg::with_name("config")
                                    .short("c")
                                    .long("config")
                                    .value_name("FILE")
                                    .help("Sets a custom config file")
                                    .takes_value(true))
                        .get_matches();

    // We can find out whether or not debugging was turned on
    if matches.is_present("debug") {
        println!("Debugging is turned on");
    }

    // If we wanted to some custom initialization based off some configuration file provided
    // by the user, we could get the file (A string of the file)
    if let Some(ref file) = matches.value_of("config") {
        println!("Using config file: {}", file);
    }
    if let Some(ref env) = matches.value_of("env") {
        println!("Using config file: {}", env);
    }
    // Continued program logic goes here...
    loader::deligate(matches)
}
