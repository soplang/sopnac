use std::env;
use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Isticmaal: sopnac <file.sop>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let code = fs::read_to_string(filename)?;
    println!("Waqti aan ka shaqeyno koodhka: {}", filename);
    println!("Waxaa ku jira:\n{}", code);

    Ok(())
}
