use std::env;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("You are at {}", path.display());
    Ok(())
}
