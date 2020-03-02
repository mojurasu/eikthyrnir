use colored::Colorize;
use git2::Repository;
use regex::Regex;
use structopt::StructOpt;

use crate::error::Result;
use crate::utils::{padded_message, print_error};

/// Helpers for git repositories
#[derive(StructOpt, Debug)]
pub enum Git {
    /// Open the assumed website from the origin
    Web {
        remote: Option<String>,
    },
}

fn get_web_url<T: Into<String>>(remote_url: T) -> Result<String> {
    let protocol = Regex::new(r#"(https?://)?(\w+@)?"#)?;
    let suffix = Regex::new(r#"\.git$"#)?;
    let url = suffix.replace(&protocol.replace(&remote_url.into(), "").to_string(), "").replace(":", "/");
    Ok(format!("http://{}", url))
}

// TODO: Open with the current branch (can be disabled with a flag)
fn web(remote: Option<String>) -> Result<()> {
    let repo = match Repository::open(".") {
        Ok(r) => r,
        Err(_) => {
            print_error("Not in a valid git repository");
            return Ok(());
        }
    };
    let remote_name = match remote {
        Some(r) => r,
        None => "origin".to_string()
    };
    let remote = match repo.find_remote(&remote_name) {
        Ok(r) => r,
        Err(_) => {
            print_error(format!("remote `{}` does not exist", remote_name));
            return Ok(());
        }
    };
    // If your remote url isnt valid UTF-8 you're doing something wrong
    let url = match remote.url() {
        Some(url) => url,
        None => {
            print_error("remote url not valid UTF-8");
            return Ok(());
        }
    };
    let web_url = get_web_url(url)?;
    padded_message("Opening".bright_purple().bold(), &web_url);
    open::that(&web_url)?;
    Ok(())
}

pub fn git(opt: Git) -> Result<()> {
    match opt {
        Git::Web { remote } => web(remote)
    }?;
    Ok(())
}

#[cfg(test)]
mod web {
    use super::*;

    mod github {
        use super::*;

        #[test]
        fn ssh_url() -> Result<()> {
            assert_eq!(get_web_url("git@github.com:mojurasu/eikthyrnir.git")?,
                       "http://github.com/mojurasu/eikthyrnir".to_string());
            Ok(())
        }

        #[test]
        fn https_url() -> Result<()> {
            assert_eq!(get_web_url("https://github.com/mojurasu/eikthyrnir.git")?,
                       "http://github.com/mojurasu/eikthyrnir".to_string());
            Ok(())
        }
    }

    mod gitlab {
        use super::*;

        #[test]
        fn ssh_url() -> Result<()> {
            assert_eq!(get_web_url("git@gitlab.com:mojurasu/eikthyrnir.git")?,
                       "http://gitlab.com/mojurasu/eikthyrnir".to_string());
            Ok(())
        }

        #[test]
        fn https_url() -> Result<()> {
            assert_eq!(get_web_url("https://gitlab.com/mojurasu/eikthyrnir.git")?,
                       "http://gitlab.com/mojurasu/eikthyrnir".to_string());
            Ok(())
        }
    }

    mod bitbucket {
        use super::*;

        #[test]
        fn ssh_url() -> Result<()> {
            assert_eq!(get_web_url("git@bitbucket.org:mojurasu/eikthyrnir.git")?,
                       "http://bitbucket.org/mojurasu/eikthyrnir".to_string());
            Ok(())
        }

        #[test]
        fn https_url() -> Result<()> {
            assert_eq!(get_web_url("https://SitiSchu@bitbucket.org/mojurasu/eikthyrnir.git")?,
                       "http://bitbucket.org/mojurasu/eikthyrnir".to_string());
            Ok(())
        }
    }

    mod gitea {
        use super::*;

        #[test]
        fn ssh_url() -> Result<()> {
            assert_eq!(get_web_url("git@gitea.example.com:mojurasu/eikthyrnir.git")?,
                       "http://gitea.example.com/mojurasu/eikthyrnir".to_string());
            Ok(())
        }

        #[test]
        fn https_url() -> Result<()> {
            assert_eq!(get_web_url("https://gitea.example.com/mojurasu/eikthyrnir.git")?,
                       "http://gitea.example.com/mojurasu/eikthyrnir".to_string());
            Ok(())
        }
    }
}
