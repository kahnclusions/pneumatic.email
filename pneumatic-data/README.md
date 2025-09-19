# pneumatic-data

This crate provides a common Rust interface for interacting with Pneumatic's Sqlite3 database, so that both the native app (Tauri), or the webapp (Browsers), can access the database. In browsers this is intended to be compiled to WASM and run in a web worker.

The frontend should include a command library that abstracts away the backend so that a common set of commands can be sent both via Tauri or web worker.

> TODO: sqlx currently cannot build on WASM, so we need to use `https://github.com/Spxg/sqlite-wasm-rs` when the target is `wasm32-unknown-unknown`. This isn't implemented yet. There is a [draft PR](https://github.com/launchbadge/sqlx/pull/3994) for this in the sqlx repo. TBD how to implement the WASM side in this lib.

```
                                ┌────────────┐                                         
                                │            │                                         
                          ┌────►│ Web Worker ├───────┐                                 
                          │     │            │       ▼                                 
┌──────────┐    ┌─────────┴───┐ └────────────┘      ┌────────────────┐    ┌───────────┐
│          │    │             │                     │                │    │           │
│ Frontend ├───►│ Command lib │                     │ pneumatic-data ├───►│ Sqlite DB │
│          │    │             │                     │                │    │           │
└──────────┘    └─────────┬───┘ ┌───────────────┐   └────────────────┘    └───────────┘
                          │     │               │    ▲                                 
                          └────►│ Tauri process ├────┘                                 
                                │               │                                      
                                └───────────────┘                                      
```

