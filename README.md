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
