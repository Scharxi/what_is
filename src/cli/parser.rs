use clap::{arg, command, ArgMatches};

pub struct CLI;

impl CLI {
    pub fn init() -> ArgMatches {
        command!()
            .arg(arg!(-w --word <WORD>).required(true))
            .get_matches()
    }
}
