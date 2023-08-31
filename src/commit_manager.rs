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

pub fn scan_for_secrets(tree: &Tree, repo: &Repository) -> Option<Vec<FoundSecret>> {
    let mut file_contents: HashMap<String, Vec<u8>> = HashMap::new();

    tree.walk(git2::TreeWalkMode::PreOrder, |_, entry| {
        let name = entry.name().unwrap_or_default();
        let object = entry.to_object(&repo).unwrap();

        if let ObjectType::Blob = object.kind().unwrap() {
            let blob = object.as_blob().unwrap();
            file_contents.insert(
                name.to_string(),
                blob.content().to_vec(),
            );
        }

        true
    }).expect("Failed to walk the tree");

    let secrets = ["password", "token", "secret"];
    let mut found_secrets: Vec<FoundSecret> = Vec::new();

    for (file, content) in file_contents.iter() {
        let file_content_str = String::from_utf8_lossy(content);

        for (line_number, line) in file_content_str.lines().enumerate() {
            for secret in secrets.iter() {
                if line.contains(secret) {
                    found_secrets.push(FoundSecret {
                        file: file.clone(),
                        line_number: line_number + 1, // line_number is zero-based
                        line_content: line.to_string(),
                        secret_keyword: secret.to_string(),
                    });
                }
            }
        }
    }

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