use crate::parser::{DynamicConfig, load_config};
use notify::{RecommendedWatcher, RecursiveMode, Config, Event, Watcher};
use std::{path::PathBuf, sync::{Arc, RwLock}};
use crossbeam_channel::unbounded;
use std::path::Path;

pub struct ConfigWatcher {
    path: PathBuf,
    config: Arc<RwLock<DynamicConfig>>,
}

impl ConfigWatcher {
    pub fn new(path: &str) -> Self {
        let cfg = load_config(path).expect("failed to load config");
        let config = Arc::new(RwLock::new(cfg));

        let (tx, rx) = unbounded();
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, _>| { let _ = tx.send(res); },
            Config::default(),
        ).expect("failed to create watcher");

        watcher.watch(Path::new(path), RecursiveMode::NonRecursive)
            .expect("failed to watch file");

        let cfg_ref = Arc::clone(&config);
        let path_buf = PathBuf::from(path);

        std::thread::spawn(move || {
            for res in rx.iter() {
                if res.is_ok() {
                    if let Ok(new_cfg) = load_config(path_buf.to_str().unwrap()) {
                        *cfg_ref.write().unwrap() = new_cfg;
                        println!("♻️  Config reloaded: {}", path_buf.display());
                    }
                }
            }
        });

        Self { path: PathBuf::from(path), config }
    }
    

    pub fn get(&self) -> Arc<RwLock<DynamicConfig>> {
        Arc::clone(&self.config)
    }
}
