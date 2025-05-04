mod compiler;

use std::env;
use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Isticmaal: sopnac <file.sop>");
        std::process::exit(1);
    }

    let input = &args[1];
    let code = fs::read_to_string(input)?;
    println!("Keenaya koodhka: {}", input);

    compiler::compile_to_binary(&code, "output.o")?;
    println!("Waxaa la sameeyay: output.o (object file)");

    // Link to executable (requires system linker like `cc`)
    std::process::Command::new("cc")
        .args(&["output.o", "-o", "output"])
        .status()?;

    println!("✅ Faylka la fulin karo: ./output");

    Ok(())
}
