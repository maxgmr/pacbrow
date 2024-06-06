use std::{io, process::Command};

use regex::Regex;

use crate::app::Package;

pub fn get_package_list() -> io::Result<Vec<Package>> {
    let output = Command::new("pacman").arg("-Qil").output()?;
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
            let info = lines.collect::<Vec<&str>>().join("\n");
            package_vec.push(Package {
                name: name.to_string(),
                info,
            });
        }
    }

    Ok(package_vec)
}
