extern crate clap;
extern crate rand;

mod filehose;

use filehose::spam;
use clap::{Arg, App};
use std::path::PathBuf;


fn main() {
    let matches = App::new("thRaSher")
                        .version("0.1")
                        .author("Bret Mattingly <bret.mattingly@gmail.com>")
                        .about("Random high-speed file creation for application testing")
                        .arg(Arg::with_name("DIRECTORY")
                            .help("The root directory in which to create files")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("COUNT")
                            .help("Number of files to create")
                            .required(true)
                            .index(2))
                        .arg(Arg::with_name("EXTENSION")
                            .help("File extension to use")
                            .required(true)
                            .index(3))
                        .get_matches();

    let directory = matches.value_of("DIRECTORY").unwrap_or("./");
    let count_str = matches.value_of("COUNT").unwrap_or("1000");
    let count: i32 = count_str.parse().unwrap();
    let ext = matches.value_of("EXTENSION").unwrap_or(".txt");

    spam(PathBuf::from(directory), count, ext);

}
