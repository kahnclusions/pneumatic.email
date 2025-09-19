# 📮 Pneumatic Email


**⚠️ Danger: very early WIP, do not use.**

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

* `pneumatic-app` the SolidJS+Vike web frontend.
* `pneumatic-data` library abstracting around the Sqlite3 database.
* `pneumatic-jmap` library abstracting around JMAP services.
* `pneumatic-tauri` the Tauri native application.


## License

This project is licensed under the _GNU Affero General Public License, Version 3_.

## Kudos

Shout-out to [Readest](https://github.com/readest/readest) which is an excellent ebook reader app also built with Tauri that serves as an amazing example.
