mod git_manager;
mod commit_manager;

use std::{io, process};
use std::error::Error;
use clap::{Parser, Subcommand};

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
            let repo = git_manager::get_git_repo();
            let head = commit_manager::get_head(&repo);
            let tree = commit_manager::get_tree(&head);
            let diff = commit_manager::get_diff(&tree, &repo);
            let (changed_files, changed_lines) = commit_manager::inspect_diff(&diff);
            println!("Changed Files: {} - Changed Lines: {}", changed_files, changed_lines);

            if commit_is_illegal(changed_files, changed_lines) {
                let should_force = handle_illegal_commit();
                if should_force.trim() != "f" {
                    //Fail the commit
                    println!("Failing Commit");
                    process::exit(1);
                }
                println!("Forcing the commit");
            }

            println!("Committing");
            //continue with the commit
            let commit_result = git_manager::commit_to_repo(&name.as_ref().unwrap_or(&"No Message provided".to_string()), "Jarl Due", "jarl.due@sos.eu");
            match commit_result {
                Ok(_) => {println!("Commit pushed successfully")}
                Err(err) => {println!("Something went wrong! {:?}", err)}
            }
        }
    }
}

fn commit_is_illegal(changed_files: i32, changed_lines: i32) -> bool {
    changed_files > 10 || changed_lines > 80
}

fn handle_illegal_commit() -> String {
    println!("This commit exceeds the maximum allowed in either lines or files");
    let mut should_force = String::new();
    println!("Enter (f) to force commit regardless");
    io::stdin().read_line(&mut should_force).expect("Failed to read line");
    should_force
}


