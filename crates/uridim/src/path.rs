use std::path::PathBuf;

pub fn print_paths() -> std::io::Result<()> {
    let current_directory: PathBuf = std::env::current_dir()?;

    for directory in current_directory.ancestors() {
        println!("{}", directory.display());
    }

    Ok(())
}
