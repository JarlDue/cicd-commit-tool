use std::error::Error;
use git2::{Repository, Signature};

pub fn get_git_repo() -> Repository {
    Repository::open(".").expect("Failed to open repo")
}

pub fn commit_to_repo(message: &str, committer_name: &str, commiter_email: &str) -> Result<(), Box<dyn Error>> {
    let repo = Repository::open(".")?;

    // Create an index object representing the Git index
    let mut index = repo.index()?;

    // Add all files to the index
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // Create a tree object from the index
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;

    // Create a commit signature
    let signature = Signature::now(committer_name, commiter_email)?;

    // Get the reference to HEAD
    let head = repo.head()?;
    let parent_commit = repo.find_commit(head.target().unwrap())?;

    // Create the commit
    let _commit = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
    )?;

    // Push the changes
    let mut remote = repo.find_remote("origin")?;
    remote.push(&["refs/heads/main:refs/heads/main"], None)?;

    Ok(())
}