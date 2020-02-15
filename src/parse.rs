extern crate clap;
use clap::{App, Arg, ArgMatches,Error};


pub fn parse(cmd: &str) -> (String, Result<ArgMatches, Error>) {
    let RULES: App = App::new("meorslib").args(&vec![
        Arg::with_name("radius")
            .long("radius")
            .short("r")
            .takes_value(true)
            .help("Radius of structure"),
        Arg::with_name("height")
            .short("h")
            .takes_value(true)
            .help("Height of structure"),
        Arg::with_name("length")
            .short("l")
            .takes_value(true)
            .help("Length of structure"),
        Arg::with_name("width")
            .short("w")
            .takes_value(true)
            .help("Width of structure"),
        Arg::with_name("facing")
            .short("f")
            .takes_value(true)
            .help("Facing of structure.(x, y or z)")
    ]);
    let spl_cmd = cmd.split_whitespace();
    let vec_cmd = spl_cmd.collect::<Vec<&str>>();
    let cfg = RULES.get_matches_from_safe(vec_cmd.clone());
    println!("{:?}", vec_cmd);
    (vec_cmd[0].to_string(), cfg)
}
