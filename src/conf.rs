use std::borrow::Borrow;
use std::env;

/******************************************************************************
Exposing the config values as a trait allows the code to be built to use either
an implementation that queries the env vars or a stubbed version for unit
testing. The `get_conf` function uses a very crude method of determining which
implementation to return.
******************************************************************************/
pub trait ConfValues {
    fn github_token(&self) -> String;
    fn allowed_repos(&self) -> Vec<String>;
}

pub fn get_conf(should_mock: bool) -> Box<dyn ConfValues> {
    if should_mock {
        return Box::new(UnitTestConf {});
    }

    Box::new(EnvConf {})
}

/******************************************************************************
Query the environment vars for config values
******************************************************************************/
struct EnvConf;

impl ConfValues for EnvConf {
    fn github_token(&self) -> String {
        env::var("RUST_WORKSHOP_TOKEN")
            .unwrap_or("DEFAULT".to_owned())
    }

    fn allowed_repos(&self) -> Vec<String> {
        env::var("RUST_WORKSHOP_REPOS")
            .unwrap_or("vector,pipeline-service".to_owned())
            .split(",")
            .map(|s| s.trim().to_owned())
            .collect::<Vec<_>>()
    }
}

/******************************************************************************
Hard coded config values for unit testing
******************************************************************************/
struct UnitTestConf;

impl ConfValues for UnitTestConf {
    fn github_token(&self) -> String {
        "UNIT TEST TOKEN".to_owned()
    }

    fn allowed_repos(&self) -> Vec<String> {
        vec!["repo_a".to_owned(), "repo_b".to_owned()]
    }
}