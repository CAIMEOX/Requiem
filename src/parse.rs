extern crate clap;
use clap::{AppSettings, Clap};


#[derive(Clap, Debug, PartialEq, Eq)]
struct Opt {
    #[clap()]
    radius: u32,
    height: u32
}

pub fn parse() {
    let opts: Opt = Opt::parse_from(&["c","d","d"]);

}