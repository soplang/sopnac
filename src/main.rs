mod compiler;

use anyhow::{bail, Result};
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    /* --------- simple CLI parsing --------- */
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        bail!("Usage: sopnac <file.sop> [-o output_name]");
    }

    let source = args.remove(0);
    let output = if args.len() >= 2 && args[0] == "-o" {
        args.remove(0);            // remove "-o"
        args.remove(0)             // take name after -o
    } else {
        // default: same name without extension
        Path::new(&source)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .into()
    };

    /* --------- compile --------- */
    let code = fs::read_to_string(&source)?;
    compiler::compile_and_link(&code, &output)?;
    println!("✅ executable generated: ./{}", output);
    Ok(())
}
