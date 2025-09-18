# pneumatic-data

Whether running in a Tauri native app, or as a PWA / browser web app, this crate provides a common Rust interface for interacting with the Sqlite3 database. On Tauri the backend can access the database as usual, and on PWA/Browsers the app will run this crate using WASM in a web worker.

The frontend library for sending commands to backend should be backend-agnostic such that either the Tauri backend or the Web Worker backend can be used.

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
