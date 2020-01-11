pub struct Circumstances {
    pub repository_directory: String,
}

impl Circumstances {
    pub fn new(repo_dir: String) -> Circumstances {
        Circumstances {
            repository_directory: repo_dir,
        }
    }
}
