use git_snapshot::Repo;
use std::env::current_dir;

fn main() {
    let cwd = current_dir().unwrap();
    let mut repo = Repo::new(cwd, None).unwrap();
    repo.snapshot().unwrap();
}
