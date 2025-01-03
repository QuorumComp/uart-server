use clap::*;

extern crate toml;
extern crate dirs;

use super::config;

#[derive(Debug)]
pub struct Arguments {
    pub port: Option<String>,
    pub path: String,
    pub debug: bool
}

impl Arguments {
    fn app() -> clap::App<'static> {
        return clap_app!(uartfileserver =>
            (@arg PORT: -p --port +takes_value "Serial port to use")
            (@arg DEBUG: --debug "Prints diagnostic messages")
            (@arg PATH: +takes_value "The path to serve as filesystem (default current directory)"))
            .author(crate_authors!())
            .version(crate_version!())
            .about(crate_description!());
    }

    pub fn new(config: config::Config) -> Arguments {
        let matches = Arguments::app().get_matches();

        let path = matches.value_of("PATH").map(|s| s.to_string()).or(config.path).unwrap_or(".".to_string());
        let port = matches.value_of("PORT").map(|s| s.to_string()).or(config.port);
        let debug = matches.is_present("DEBUG");

        return Arguments { port: port, debug: debug, path: path.to_string() };
    }
}

