use uridim::git::find_git_root;

fn main() -> std::io::Result<()> {
    let current_directory = std::env::current_dir()?;

    match find_git_root(&current_directory)? {
        Some(candidate) => println!("{candidate:#?}"),
        None => println!("No Git repository boundary found"),
    }

    Ok(())
}
