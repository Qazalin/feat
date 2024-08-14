use std::env;
use std::error::Error;

fn parse(o: std::process::Output) -> String {
    std::str::from_utf8(&o.stdout).unwrap().trim().to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();
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
    println!("{}", repo);
    Ok(())
}
