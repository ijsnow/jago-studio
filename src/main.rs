extern crate circumstance;

use circumstance::Circumstances;
use git2::Repository;
use std::env;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Debug)]
enum JagoError {
    Parse(url::ParseError),
    Repository(git2::Error),
    InvalidInput,
}

impl fmt::Display for JagoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JagoError::Parse(ref err) => err.fmt(f),
            JagoError::Repository(ref err) => err.fmt(f),
            JagoError::InvalidInput => write!(f, "invalid remote repository url"),
        }
    }
}

impl Error for JagoError {
    fn description(&self) -> &str {
        match *self {
            JagoError::Parse(ref err) => err.description(),
            JagoError::Repository(ref err) => err.description(),
            JagoError::InvalidInput => "invalid input",
        }
    }
}

impl From<url::ParseError> for JagoError {
    fn from(err: url::ParseError) -> JagoError {
        JagoError::Parse(err)
    }
}

impl From<git2::Error> for JagoError {
    fn from(err: git2::Error) -> JagoError {
        JagoError::Repository(err)
    }
}

fn main() {
    let home_dir = std::env::var_os("HOME")
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let context = Circumstances::new(home_dir + "/src");

    let args: Vec<String> = env::args().collect();

    let remote = &args[1];

    match clone_repo(context, remote) {
        Ok(repo_path) => {
            println!("Repository successfully cloned to {:?}", repo_path);
        }
        Err(e) => {
            println!("Failed to clone {:?}. Error: {}", remote, e);
        }
    }
}

fn clone_repo(context: Circumstances, remote: &str) -> Result<PathBuf, JagoError> {
    let dest = get_destination_dir(context, remote)?;
    println!("Cloning to {:?}...", dest);
    Repository::clone(remote, &dest)?;

    Ok(dest)
}

fn get_destination_dir(circumstances: Circumstances, remote: &str) -> Result<PathBuf, JagoError> {
    let u = match Url::parse(remote) {
        Ok(url) => url,
        Err(e) => return Err(JagoError::Parse(e)),
    };

    let host = match u.host_str() {
        Some(u) => u,
        None => return Err(JagoError::InvalidInput),
    };

    let path = u.path();

    let mut dest = Path::new(&circumstances.repository_directory)
        .join(host)
        .join(&path[1..]);

    dest.set_extension("");

    Ok(dest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_destination_dir_works() {
        let context = Circumstances::new("/tmp".to_string());
        let dest = get_destination_dir(context, "https://github.com/xi-editor/xi-editor.git");

        assert_eq!(
            dest.unwrap().to_str().unwrap(),
            "/tmp/github.com/xi-editor/xi-editor"
        );

        let context = Circumstances::new("/tmp".to_string());
        let dest = get_destination_dir(context, "git@github.com:xi-editor/xi-editor.git");

        assert_eq!(
            dest.unwrap().to_str().unwrap(),
            "/tmp/github.com/xi-editor/xi-editor"
        );
    }
}
