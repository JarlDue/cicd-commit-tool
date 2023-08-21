use clap::{Parser, Subcommand};
use git2::Repository;

#[derive(Parser)]
#[command(author = "Jarl Due. <jarl.due@sos.eu")]
#[command(name = "CICD Commit Helper")]
#[command(version = "0.1")]
#[command(about = "A small CLI project to help ease good commit policies")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Commit { name: Option<String> }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Commit { name } => {
            println!("Reff {name:?}");
            let repo = get_git_repo();
        }
    }
}

fn get_git_repo() -> Repository {
    git2::Repository::open(".").expect("Failed to open repo")    
}
