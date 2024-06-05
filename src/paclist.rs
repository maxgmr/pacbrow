use std::{
    io,
    process::Command,
    time::{Duration, Instant},
};

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
    let name_re = Regex::new(r"(?m)^\S+$").unwrap();
    let info_re = Regex::new(r"(?m)^.*$\n((?:.*\n)+.+)").unwrap();

    let mut package_vec: Vec<Package> = Vec::new();

    let mut name_re_cuml = Duration::new(0, 0);
    let mut name_from_cuml = Duration::new(0, 0);
    let mut info_re_cuml = Duration::new(0, 0);
    let mut info_from_cuml = Duration::new(0, 0);
    let mut info_fmt_cuml = Duration::new(0, 0);
    let mut push_cuml = Duration::new(0, 0);

    for raw_entry in split_re.split(&raw_string) {
        let mut lines = raw_entry.lines();
        if let Some(name) = lines.next() {
            let info = format!(
                "{}\n=======\n{}",
                name,
                lines.collect::<Vec<&str>>().join("\n")
            );
            package_vec.push(Package {
                name: name.to_string(),
                info,
            });
        }

        // let nrs = Instant::now();
        // if let Some(name_caps) = name_re.captures(raw_entry) {
        //     name_re_cuml += nrs.elapsed();
        //     let nfs = Instant::now();
        //     let name = String::from(&name_caps[0]);
        //     name_from_cuml += nfs.elapsed();
        //     let irs = Instant::now();
        //     let info_string = if let Some(info_caps) = info_re.captures(raw_entry) {
        //         info_re_cuml += irs.elapsed();
        //         let ifs = Instant::now();
        //         let s = String::from(&info_caps[1]);
        //         info_from_cuml += ifs.elapsed();
        //         s
        //     } else {
        //         info_re_cuml += irs.elapsed();
        //         let ifs = Instant::now();
        //         let s = String::new();
        //         info_from_cuml += ifs.elapsed();
        //         s
        //     };
        //     let ifs = Instant::now();
        //     let info = format!("{name}\n=======\n{info_string}");
        //     info_fmt_cuml += ifs.elapsed();
        //     let ps = Instant::now();
        //     package_vec.push(Package { name, info });
        //     push_cuml += ps.elapsed();
        // } else {
        //     name_re_cuml += nrs.elapsed();
        // }
    }
    // dbg!(
    //     name_re_cuml,
    //     name_from_cuml,
    //     info_re_cuml,
    //     info_from_cuml,
    //     info_fmt_cuml,
    //     push_cuml
    // );
    Ok(package_vec)
}
