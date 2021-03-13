use druid::{Data, Lens};

use std::env;
use std::path::{Path, PathBuf};
use std::{sync::Arc};

/// @FIXME: decouple druid from yukari_lib
#[derive(Clone, Data, Lens)]
pub struct State {
    #[data(same_fn = "PartialEq::eq")]
    pub parent: Option<PathBuf>,
    #[data(same_fn = "PartialEq::eq")]
    pub current: PathBuf,
    #[data(ignore)]
    current_content: Arc<Vec<String>>,
    #[data(ignore)]
    parent_content: Arc<Vec<String>>,
}

impl State {
    /// falls back to home
    pub fn cwd() -> Self {
        let current = env::current_dir()
            .or_else(|_| env::home_dir().ok_or(()))
            .unwrap_or("/".into());
        let parent = current.parent().map(|p| p.to_owned());
        let mut state = Self {
            current,
            parent,
            current_content: Default::default(),
            parent_content: Default::default(),
        };
        state.update();
        state
    }

    pub fn update(&mut self) {
        self.current_content = Arc::new(Self::dir_contents(&self.current));
        self.parent_content = Arc::new(self.parent.as_ref().map_or(vec![], Self::dir_contents));
    }

    pub fn dir_contents(path: impl AsRef<Path>) -> Vec<String> {
        match path.as_ref().read_dir().and_then(|it| {
            it.map(|r| r.map(|de| de.file_name().to_string_lossy().into_owned()))
                .collect()
        }) {
            Ok(cs) => cs,
            Err(_) => vec![],
        }
    }
}

struct DirInfo {
    num_children: u32,
}