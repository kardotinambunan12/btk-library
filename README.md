# configx

`configx` adalah library Rust untuk **dynamic configuration** dengan dukungan:

- YAML / TOML / JSON
- Auto-reload saat file berubah
- Environment variable expansion `${VAR}`
- Ambil parameter spesifik dengan dot-access: `get("a.b.c")`

---

## 📦 Instalasi

Jika library sudah dipublish ke crates.io:

```toml
[dependencies]
configx = "0.1"


```

## Dukungan Format
```toml
.yaml / .yml → YAML

.toml → TOML

.json → JSON

```

## Contoh Penggunaan
```toml
use configx::ConfigWatcher;
use std::{thread, time::Duration};

fn main() {
    // Buat watcher untuk config.yaml (atau .json/.toml)
    let watcher = ConfigWatcher::new("config.yaml");
    let cfg_ref = watcher.get();

    loop {
        let cfg = cfg_ref.read().unwrap();

        //cara mengambil parameter
        if let Some(host) = cfg.get("server.host").and_then(|v| v.as_str().map(|s| s.to_string())) {
            println!("Server host: {}", host);
        }

        if let Some(port) = cfg.get("server.port").and_then(|v| v.as_u64()) {
            println!("Server port: {}", port);
        }

        if let Some(db_url) = cfg.get("database.url").and_then(|v| v.as_str().map(|s| s.to_string())) {
            println!("Database URL: {}", db_url);
        }

        // Delay untuk demo auto-reload
        thread::sleep(Duration::from_secs(5));
    }
}

```