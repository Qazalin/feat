#![allow(deprecated)]
use std::{env, error::Error, fs, io::Write};

fn parse(o: std::process::Output) -> String {
    std::str::from_utf8(&o.stdout).unwrap().trim().to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let fp = env::home_dir().unwrap().join(".feat");
    let name = env::args().last().unwrap();
    if name == "clean" {
        fs::File::create(fp)?;
        return Ok(());
    }
    let branch = parse(
        std::process::Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()?,
    );
    let repo = parse(
        std::process::Command::new("git")
            .arg("remote")
            .arg("get-url")
            .arg("origin")
            .output()?,
    );
    let key = format!("{name}:{repo}");
    match branch == "master" {
        true => {
            let feats = fs::read_to_string(&fp)?;
            match feats.lines().find(|x| x.starts_with(&key)) {
                Some(val) => {
                    let val = val.split(":").last().expect("{x}");
                    let checkout = std::process::Command::new("git")
                        .arg("checkout")
                        .arg(val)
                        .output();
                    if checkout.is_err() {
                        std::process::Command::new("git").arg("stash").output()?;
                        std::process::Command::new("git")
                            .arg("checkout")
                            .arg(val)
                            .output()?;
                    }
                    Ok(())
                }
                None => Err(format!("not feature set for {key}")),
            }?
        }
        false => {
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&fp)?;
            writeln!(file, "{}:{}", key, branch)?;
        }
    }

    Ok(())
}
