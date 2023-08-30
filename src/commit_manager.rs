use git2::{Commit, Diff, Repository, Tree};

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