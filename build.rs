//! A special file that runs before compilation to generate code, set compilation flags, env variables
//! or link to native libraries
//!
use std::process::Command;

fn main() {
    let hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"]) // get HEAD short commit
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output() // execute command on a child process
        .map(|o| String::from_utf8(o.stdout).unwrap());
    let date = Command::new("git")
        .args(["log", "--pretty=format:'%ad'", "-n1", "--date=short"])
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .map(|o| String::from_utf8(o.stdout).unwrap());
    if let (Ok(hash), Ok(date)) = (hash, date) {
        let ver = format!(
            "{} (commit {} {})",
            env!("CARGO_PKG_VERSION"),
            hash.trim(),
            date.trim_matches('\'')
        );
        println!("cargo:rustc-env=VERSION={ver}"); // set new env variable
    } else {
        println!("cargo:rustc-env=VERSION={}", env!("CARGO_PKG_VERSION"));
    }
}
