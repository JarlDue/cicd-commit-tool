use std::collections::HashMap;
use git2::{Commit, Diff, DiffHunk, ObjectType, Repository, Tree};

pub fn inspect_diff(diff: &Diff) -> (i32, i32) {
    let mut number_files_changed = 0;
    let mut number_of_lines_changed = 0;

    for i in 0..diff.deltas().len() {
        number_files_changed += 1;

        let patch = git2::Patch::from_diff(&diff, i).expect("Failed to get patch");
        if let Some(patch) = patch {
            for hunk_index in 0..patch.num_hunks() {
                let (_hunk, lines) = patch.hunk(hunk_index).expect("Failed to get hunk");
                for line_index in 0..lines {
                    let line = patch.line_in_hunk(hunk_index, line_index).expect("Failed to get line");
                    match line.origin() {
                        '+' | '-' => number_of_lines_changed += 1,  // counting only added and removed lines
                        _ => {}
                    }
                }
            }
        }
    }
    (number_files_changed, number_of_lines_changed)
}

pub fn get_head(repository: &Repository) -> Commit {
   repository.head().expect("Failed to fetch HEAD").peel_to_commit().expect("Failed to peel to commit")
}

pub fn get_tree<'a>(commit: &'a Commit) -> Tree<'a> {
    commit.tree().expect("Failed to fetch tree")
}

pub fn get_diff<'a>(tree: &'a Tree, repository: &'a Repository) -> Diff<'a> {
   repository.diff_tree_to_workdir_with_index(Some(&tree), None).expect("Failed to get diff")
}

pub fn scan_for_secrets(diff: &Diff, tree: &Tree, repo: &Repository) -> Option<Vec<FoundSecret>> {
    let secrets = ["password", "token", "secret"];
    let mut found_secrets = Vec::new();

    // Iterate through each diff to find the changed files
    diff.foreach(&mut |delta, _| {
        if let Some(file_path) = delta.new_file().path() {
            if let Ok(object) = tree.get_path(file_path) {
                if let ObjectType::Blob = object.kind().unwrap() {
                    let blob_object = object.to_object(&repo).unwrap();
                    let blob = blob_object.as_blob().unwrap(); // This should work now
                    let content = blob.content();

                    let secrets = ["password", "token", "secret"];
                    for (line_number, line) in String::from_utf8_lossy(content).lines().enumerate() {
                        for &secret in &secrets {
                            if line.contains(secret) {
                                found_secrets.push(FoundSecret {
                                    file: file_path.to_str().unwrap().to_string(),
                                    line_number: line_number + 1,
                                    line_content: line.to_string(),
                                    secret_keyword: secret.to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
        true
    }, None, None, None).expect("Error iterating through diff");

    if found_secrets.is_empty() {
        None
    } else {
        Some(found_secrets)
    }
}

#[derive(Debug)]
pub struct FoundSecret {
    pub file: String,
    pub line_number: usize,
    pub line_content: String,
    pub secret_keyword: String,
}