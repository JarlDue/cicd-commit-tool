use std::env;
use std::error::Error;
use git2::{Cred, PushOptions, RemoteCallbacks, Repository, Signature};

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

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        println!("Attempting to authenticate for URL: {}", _url);
        let password = env::var("GIT_PASSWORD").expect("GIT_PASSWORD not set");
        println!("{:?}", username_from_url);
        println!("{:?}", password);
        let username = username_from_url.unwrap_or("Some-Mail");
        println!("{:?}", username);
        Cred::userpass_plaintext(&username, &password)
    });

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    println!("Pushing");
    // Push the changes
    let mut remote = repo.find_remote("origin")?;
    println!("Remote {:?}", remote.url());
    remote.push(&["refs/heads/main:refs/heads/main"], Some(&mut push_options))?;

    Ok(())
}