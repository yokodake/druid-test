use std::path::PathBuf;
use std::env;

pub struct State {
    parent: Option<PathBuf>,
    current: PathBuf,
}

impl State {
    fn cwd() -> Self {
        let current =
            env::current_dir()
            .or_else(|_| env::home_dir().ok_or(()))
            .unwrap_or("/".into());
        let parent = None;
        Self { parent, current }
    }
}
