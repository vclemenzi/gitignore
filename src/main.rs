use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return println!("Usage: gitignore <template>");
    }

    let content = reqwest::blocking::get(format!(
        "https://raw.githubusercontent.com/github/gitignore/main/{}.gitignore",
        args[1]
    ))
    .unwrap();

    if content.status() == 404 {
        return println!("This template dosen't exist!")
    }

    // Create .gitignore

    let mut file = fs::File::create("./.gitignore").unwrap();

    file.write_all(content.text().unwrap().to_string().as_bytes())
        .expect("Error: Impossible generate the .gitignore");
}
