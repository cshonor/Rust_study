use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};

const USERNAME_FILE: &str = "hello.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "help".to_string());

    match mode.as_str() {
        "open_or_create" => {
            let mut f = open_or_create(USERNAME_FILE)?;
            writeln!(f, "username=rustacean")?;
            println!("wrote {USERNAME_FILE}");
        }
        "read_match" => {
            let username = read_username_from_file_match(USERNAME_FILE)?;
            println!("read_match: {username:?}");
        }
        "read_q" => {
            let username = read_username_from_file_q(USERNAME_FILE)?;
            println!("read_q: {username:?}");
        }
        "expect" => {
            // 演示：失败直接 panic
            let mut s = String::new();
            File::open("definitely_missing.txt")
                .expect("Failed to open definitely_missing.txt")
                .read_to_string(&mut s)
                .unwrap();
            println!("{s}");
        }
        "help" | _ => {
            eprintln!(
                "usage:\n  cargo run -- open_or_create\n  cargo run -- read_match\n  cargo run -- read_q\n  cargo run -- expect"
            );
        }
    }

    Ok(())
}

fn open_or_create(path: &str) -> Result<File, io::Error> {
    let f = File::open(path);

    match f {
        Ok(file) => Ok(file),
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => File::create(path),
            other => Err(io::Error::new(other, format!("Problem opening {path}"))),
        },
    }
}

fn read_username_from_file_match(path: &str) -> Result<String, io::Error> {
    let f = File::open(path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_q(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

#[allow(dead_code)]
fn read_username_from_file_std(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

#[allow(dead_code)]
fn append_line(path: &str, line: &str) -> Result<(), io::Error> {
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(f, "{line}")?;
    Ok(())
}

