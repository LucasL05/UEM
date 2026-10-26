use std::io::{self, Write};

fn main() {
    let input = get_input().unwrap();
    println!("{input}");
}

fn get_input() -> io::Result<String> {
    println!("Paste your code below");

    io::stdout().flush()?;

    let mut buf = String::new();

    io::stdin()
        .read_line(&mut buf)?;

    Ok(buf.trim().to_string())
}