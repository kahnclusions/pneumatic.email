# 📮 Pneumatic Email


**⚠️ Danger: very early WIP, do not use.**

Pneumatic is an e-mail, contacts and calendar app for JMAP services that runs as a native app on desktop and mobile with Tauri. Pneumatic is built using SolidJS, Vike and Tauri.

## Roadmap
* JMAP Contacts / Address books.
* Support DKIM signature validation.
* Support GPG/SMIME signature validation, encryption/decryption.
* Enable Pneumatic to run as a progressive web app (PWA), or as a standard browser web app.
* JMAP Calendar
* Support plugins: investigate WASM-based Rust plugins for backend, and TypeScript plugins for frontend.

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

* `pneumatic-app` the SolidJS+Vike web frontend.
* `pneumatic-data` library abstracting around the Sqlite3 database.
* `pneumatic-jmap` library abstracting around JMAP services.
* `pneumatic-tauri` the Tauri native application.


## License

This project is licensed under the _GNU Affero General Public License, Version 3_.

## Kudos

Shout-out to [Readest](https://github.com/readest/readest), an ebook reader app also built with Tauri, that serves as an amazing example for building isomorphic apps.
