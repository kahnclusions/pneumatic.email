# Pneumatic

**Danger: early WIP, do not use.**

Pneumatic is an e-mail, contacts and calendar app for JMAP services that can run as a native app with Tauri, as a progressive web app (PWA), or as a standard browser web app. Pneumatic is built using SolidJS, Vike and Tauri.

## Development

### Getting started

First install dependencies with Deno:

```bash
git submodule update --init
deno install
```

Then you can run the tauri app with:

```bash
deno task tauri dev
```

Or run the web app:

```bash
deno task dev:web
```

### Project structure

* `pneumatic-app` the Vike+SolidJS web frontend.
* `pneumatic-data` the data library abstracting around the Sqlite3 database.
* `pneumatic-tauri` the Tauri native application.

