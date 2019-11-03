extern crate clap;
extern crate pattern;

use clap::{App, Arg};

fn main() {
    
    let matches = App::new("pattern")
        .version("1.0.0")
        .author("encry1024 <encry1024@gmail.com>")
        .about("")
        .arg(Arg::with_name("length")
             .help("The length of payload")
             .short("l")
             .long("length")
             .takes_value(true))
        .arg(Arg::with_name("offset")
             .help("Calculate the offset with given value")
             .short("o")
             .long("offset")
             .takes_value(true))
        .arg(Arg::with_name("bigendian")
             .help("Using big endian")
             .short("b")
        ).get_matches();
    if let Some(v) = matches.value_of("length") {
        let length: usize = v.parse().unwrap();
        let payload = pattern::create(length);
        println!("{}", payload);
        std::process::exit(0);
    }
    
    if let Some(v) = matches.value_of("offset") {
        let flag = if matches.is_present("bigendian") { true } else { false };
        if let Some(i) = pattern::offset(v, flag) {
            println!("{}", i);
        } else {
            std::process::exit(-1);
        }
        std::process::exit(0);
    }
}
