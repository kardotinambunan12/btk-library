use configx::watcher::ConfigWatcher;
use std::{thread, time::Duration};

fn main() {
    let watcher = ConfigWatcher::new("config.yaml");
    let cfg_ref = watcher.get();

    loop {
        let cfg = cfg_ref.read().unwrap();
        // println!("📡 Current config:\n{}", serde_yaml::to_string(&*cfg).unwrap());
        match &*cfg {
            configx::parser::DynamicConfig::Yaml(val) => {
                println!("📡 YAML config:\n{}", serde_yaml::to_string(val).unwrap());
            }
            configx::parser::DynamicConfig::Toml(val) => {
                println!("📡 TOML config:\n{}", toml::to_string_pretty(val).unwrap());
            }
            configx::parser::DynamicConfig::Json(val) => {
                println!("📡 JSON config:\n{}", serde_json::to_string_pretty(val).unwrap());
            }
        }
//test
        // if let Some(port) = cfg.get("server.port").and_then(|v| v.as_u64()) {
        //     println!("server port: {}", port);
        // }
        // if let Some(db_url) = cfg.get("database.url").and_then(|v| v.as_str().map(|s| s.to_string())) {
        //     println!("Database URL: {}", db_url);
        // }

        thread::sleep(Duration::from_secs(5));

    }
}
