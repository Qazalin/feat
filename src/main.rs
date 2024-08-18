#![allow(deprecated)]
use std::{env, error::Error, fs, io::Write};

fn parse(o: std::process::Output) -> String {
    std::str::from_utf8(&o.stdout).unwrap().trim().to_string()
}

fn checkout(branch: String) -> Result<(), Box<dyn Error>> {
    let val = branch.split(":").last().expect("{x}");
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
    let feats = fs::read_to_string(&fp)?;
    let new_branch = feats.lines().find(|x| x.starts_with(&key));
    match branch == "master" {
        true => match new_branch {
            Some(val) => match checkout(val.to_string()) {
                Ok(_) => Ok(()),
                Err(_) => Err(format!("can't checkout {val}")),
            },
            None => Err(format!("not feature set for {key}")),
        }?,
        false => match new_branch {
            Some(val) => match checkout(val.to_string()) {
                Ok(_) => Ok(()),
                Err(_) => Err(format!("can't checkout {val}")),
            },
            None => {
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(&fp)
                    .unwrap();
                writeln!(file, "{}:{}", key, branch).unwrap();
                Ok(())
            }
        }?,
    }

    Ok(())
}
