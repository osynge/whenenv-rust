use autoconf;
use clap::App;
use clap::Arg;
use clap::ArgMatches;

pub fn cli_clap(number: &u32) -> ArgMatches {
    let application = App::new(autoconf::package())
        .about("Parses an input file to do awesome things")
        .version(autoconf::version_cli())
        .author("Owen Synge <osynge@googlemail.com>")
        .arg(
            Arg::with_name("verbose")
                .help("Increase log output.")
                .short("v")
                .multiple(true)
                .long("verbose"),
        )
        .arg(
            Arg::with_name("quiet")
                .help("Decrease log output.")
                .short("q")
                .multiple(true)
                .long("quiet"),
        )
        .arg(
            Arg::with_name("rdbms")
                .long("rdbms")
                .value_name("FILE_RDBMS")
                .help("sqlite rdbms file.")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("env")
                .short("e")
                .long("env")
                .value_name("ENVIROMENT_VARIABLE")
                .help("Which enviroment variables to process")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dir-jobs")
                .long("dir-jobs")
                .value_name("DIR_JOB")
                .help("directory storing json jobs.")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dir-sh")
                .long("dir-sh")
                .value_name("DIR_SHELL")
                .help("directory storing jobs shell scripts.")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dir-py")
                .long("dir-py")
                .value_name("DIR_PYTHON")
                .help("directory storing jobs python scripts.")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("TARGET")
                .help("Sets the target to be selected, so jobs can be matched to provides. Defaults to 'execution'.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("list-provides")
                .long("list-provides")
                .help("List all jobs sorted by the functions they provide."),
        )
        .arg(
            Arg::with_name("list-target")
                .long("list-target")
                .help("List jobs by the target they provide. Not all jobs have a target."),
        )
        .arg(Arg::with_name("session").long("session").help("session"));
    let matches = application.get_matches();
    return matches;
}
