//! cat is a recreation of the standard UNIX tool, albiet in native Rust.
//! Unlike BSD or GNU cat, this varient has no command line switches.
//!
//! Usage: cat [file ...]
//!
//! cat reads the files listed on the command line sequentially and prints
//! their contents to standards output.  If a file is "-", cat will read from
//! standard input.  Unlike BSD cat, cat does not do any special handling for
//! UNIX domain sockets.

extern crate getopts;
use getopts::{optflag,getopts};
use std::io;
use std::io::{BufferedReader, File};
use std::os;
use std::str;

fn basename<'a>(path: &'a str) -> &'a str {
    if path.ends_with("/") {
        return "";
    }
    match path.rfind('/') {
        Some(idx) => path.slice_from(idx + 1),
        None => path
    }
}

fn program() -> String {
    let ref first = os::args()[0];
    basename(first.as_slice()).to_string()
}

fn cat_stdin() {
    for line in io::stdin().lines() {
        print!("{}", line.unwrap());
    }
}

fn cat_file(name: &str) {
    let path = Path::new(name);
    let mut file = BufferedReader::new(File::open(&path));
    for line in file.lines() {
        match line {
            Ok(line) => print!("{}", line),
            Err(err) => {
                println!("{}: {}: {}", program(), name, err.desc);
                break;
            },
        }
    }
}

fn help() {
    println!("usage: {} [option] [file ...]", program());
    println!("      --help\t\tdisplay this help and exit");
    println!("      --version\t\toutput version information and exit");
}

fn version() {
    let version_contents = include_bin!("../VERSION");
    let copyright_contents = include_bin!("../COPYRIGHT");
    let version = str::from_utf8(version_contents).unwrap();
    let copyright = str::from_utf8(copyright_contents).unwrap();
    print!("{} v{}", program(), version);
    print!("{}", copyright);
}

fn main() {
    let opts = &[
        optflag("", "help", "output this help and exit"),
        optflag("", "version", "output version information and exit"),
    ];
    let matches = match getopts(os::args().tail(), opts) {
        Ok(m) => { m }
        Err(f) => {
            println!("{}: {}", program(), f);
            return;
        }
    };

    if matches.opt_present("help") {
        help();
        return;
    }
    if matches.opt_present("version") {
        version();
        return;
    }

    if matches.free.is_empty() {
        cat_stdin();
    } else {
        for filename in matches.free.iter() {
            if filename.as_slice() == "-" {
                cat_stdin();
            } else {
                cat_file(filename.as_slice());
            }
        }
    }
}
