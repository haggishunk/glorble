use std::env;

fn main() -> std::io::Result<()> {
    getcwd();
    Ok(())
}

// print the cwd
fn getcwd() {
    let d = env::current_dir().unwrap();
    println!("You are at {}", d.display());
}
