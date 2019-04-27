extern crate clap;
use clap::{Arg, App}; // SubCommand

pub fn build_cli() -> App<'static, 'static> {
    App::new("Lolcate")
        .version("0.1")
        .author("Nicolas Girard <girard.nicolas@gmail.com>")
        .about("Find files by name -- A better locate / mlocate / updatedb")
        .arg(Arg::with_name("create")
            .help("Create a database")
            .long("create")
            .takes_value(false)
            .conflicts_with_all(&["pattern", "update", "info"])
            .required(false)
            )
        .arg(Arg::with_name("info")
            .help("Display configuration informations and existing databases")
            .long("info")
            .takes_value(false)
            .conflicts_with_all(&["pattern", "update", "create", "database"])
            .required(false)
            )
        .arg(Arg::with_name("update")
            .help("Update database")
            .long("update")
            .takes_value(false)
            .conflicts_with_all(&["pattern", "create", "info"])
            .required(false)
            )
        .arg(Arg::with_name("database")
            .help("Database to be used / created")
            .long("db")
            .takes_value(true)
            .required(false)
            .default_value("default"))
        .arg(Arg::with_name("type")
            .help("One or several file types to search, separated with commas")
            .long("type")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("all")
            .help("Query / update all databases")
            .long("all")
            .takes_value(false)
            .conflicts_with_all(&["create", "info"])
            .required(false)
            )
        .arg(Arg::with_name("pattern")
            .value_name("PATTERN")
            .min_values(1)
            .required(false))
}
