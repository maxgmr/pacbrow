use clap::ArgMatches;

use std::{io, process::Command};

use regex::Regex;

use crate::app::Package;

pub fn get_package_list(cli_args: &ArgMatches) -> io::Result<Vec<Package>> {
    let mut arg_string = String::from("-Qil");
    if cli_args.get_flag("deps") {
        arg_string.push('d');
    }
    if cli_args.get_flag("explicit") {
        arg_string.push('e');
    }
    if cli_args.get_flag("foreign") {
        arg_string.push('m');
    }
    if cli_args.get_flag("native") {
        arg_string.push('n');
    }
    if cli_args.get_flag("unrequired") {
        arg_string.push('t');
    }
    if cli_args.get_flag("upgrades") {
        arg_string.push('u');
    }

    let output = Command::new("pacman").arg(arg_string).output()?;
    let raw_string = match String::from_utf8(output.stdout.to_vec()) {
        Ok(v) => v,
        Err(e) => {
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    };
    parse_package_list(raw_string)
}

fn parse_package_list(raw_string: String) -> io::Result<Vec<Package>> {
    let split_re = Regex::new(r"(?m)^Name\s+: ").unwrap();

    let mut package_vec: Vec<Package> = Vec::new();

    for raw_entry in split_re.split(&raw_string) {
        let mut lines = raw_entry.lines();
        if let Some(name) = lines.next() {
            let info = lines.map(|l| l.to_owned()).collect::<Vec<String>>();
            package_vec.push(Package {
                name: name.to_string(),
                info,
            });
        }
    }

    Ok(package_vec)
}
