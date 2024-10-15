use anyhow::Result;

fn main() {
    match run() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {:#}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    Ok(())
}
