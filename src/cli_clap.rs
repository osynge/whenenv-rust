use clap::Arg;
use clap::App;
use clap::{ArgMatches};
use loader;
use rusqlite::Connection;

pub fn cli_clap(number: &u32) -> ArgMatches{

    let application = App::new("whenenv")
                        .about("Parses an input file to do awesome things")
                        .version("0.0.1")
                        .author("Owen Synge <osynge@googlemail.com>")
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
                        .arg(Arg::with_name("dir-py")
                                    .long("dir-py")
                                    .value_name("DIR_PYTHON")
                                    .help("directory storing jobs python scripts.")
                                    .multiple(true)
                                    .takes_value(true))
                        .arg(Arg::with_name("config")
                                    .short("c")
                                    .long("config")
                                    .value_name("FILE")
                                    .help("Sets a custom config file")
                                    .takes_value(true));
    let matches = application.get_matches();

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
    //loader::deligate(conn, matches);
    return matches;
}
