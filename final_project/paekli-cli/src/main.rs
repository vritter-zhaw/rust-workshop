use anyhow::Context;
use clap::{Parser, Subcommand};
use std::fs::{self, read_to_string};
use std::{fs::read, path::Path};
/// send and receive joy with ✨ paekli-cli ✨

#[derive(Subcommand)]
enum Command {
    Send { content: String },
    Receive,
}

#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let project_dir = directories::ProjectDirs::from("dev", "buenzli", "paekli")
        .context("the user's home directory seems to be corrupt")?;
    let storage_dir = project_dir.data_dir();
    std::fs::create_dir_all(storage_dir).expect("faield to create storage directory");

    match args.command {
        Command::Send { content } => {
            if !Path::exists(&storage_dir.join("content")) {
                std::fs::write(storage_dir.join("content"), content)
                    .context("failed to store paekli")?;
            } else {
                return Err(anyhow::anyhow!("Paekli Storage is full"));
            }
        }
        Command::Receive => {
            if Path::exists(&storage_dir.join("content")) {
                match read_to_string(storage_dir.join("content")) {
                    Ok(contents) => println!("Paekli content: \n{}", contents),
                    Err(e) => println!("Error reading Paekli Storage: {}", e),
                }
                std::fs::remove_file(storage_dir.join("content"))
                    .context("failed to remove storage")?;
            } else {
                return Err(anyhow::anyhow!("Paekli Storage is empty :("));
            }
        }
    }
    Ok(())
}
