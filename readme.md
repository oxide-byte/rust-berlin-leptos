# Rust - Berlin Hack and Learn Challenge

## Purpose

This implementation is a adaptation of the Rust UserGroup **Rust Hack and Learn** Berlin 
challenge: https://github.com/andreasklostermaier/halreslib in different Frameworks.

My choice **Leptos** as Frontend.

Link to the meetup group: https://berline.rs/

Even whe most of features are implemented and functional, there is still room for progress...

## Implementation

SALT = (S)urrealDB (A)xum (L)eptos (T)ailwind + GraphQL

The Frameworks in use:

Frontend:

* Leptos 0.8.16 (CSR)
* Thaw 0.5.0-Beta
* Tailwind 4.0
* GraphQL Query + Mutation + Subscription

Backend:

* Axum 0.8.9
* Tower 0.5.3 / tower-http 0.7 (Static Page Server)
* async-graphql (GraphQL)
* jsonwebtoken 11 (JWT validation, Keycloak groups check; uses the `rust_crypto` feature)

Database:

* SurrealDB 3.3.4 (Docker image and `surrealdb` crate). The server creates the `berlin` namespace and `url_inventory` database on startup, as v3 no longer does this implicitly.

Security:

* KeyCloak, via [keycloak-wasm-auth](https://github.com/oxide-byte/keycloak-wasm-auth)

## Installation / Preparation

* Rust: https://www.rust-lang.org/tools/install (client-leptos uses the nightly toolchain with the `wasm32-unknown-unknown` target, set in `client-leptos/rust-toolchain.toml`)
* Docker (SurrealDB, KeyCloak, mailcatcher): `docker compose up -d`
* Trunk: https://trunkrs.dev/guide/getting-started/installation.html
* TailwindCss: https://tailwindcss.com/docs/installation/using-vite

PS: For TailwindCss I use the Mac Brew option:

```shell
brew update
brew install tailwindcss
```

## Running

Start the infrastructure first (SurrealDB on 8000, KeyCloak on 8888, mailcatcher on 1080):

```bash
docker compose up -d
```

The implementation uses Trunk to generate and run the Client Side Pages. It includes Hot Reloading

```bash
cd client-leptos
trunk serve --open
```

## Build

```bash
cd client-leptos
trunk build --release --public-url "/web"
```

The files generated in the client folder /dist can be moved on the
Server side to the folder /static. To access to the files from Axum
you need to manually adapt in the index.html file the relative path
for the links as the wasm file:

Example:

href="rust-berlin-leptos-7a6a9f219c87d073.js" --> href="/web/rust-berlin-leptos-7a6a9f219c87d073.js"

## Starting the Backend:

```bash
cd server
cargo run
```

The different links on the Backend side:

```text
Server on:        http://127.0.0.1:8080
Leptos client on: http://127.0.0.1:8080/web
Graphiql on:      http://127.0.0.1:8080/graphiql
Playground on:    http://127.0.0.1:8080/playground
```

## Security

There are currently 2 users implemented with KeyCloak. 

- admin@demo.com:password
- user@demo.com:password

![alt text](docs/screenshot-keycloak.png "screenshot Keycloak")

## Improvements

- Review SurrealDB specially for SQL Injection and Query Builder
- Review and Optimize code for removing .clone()
- Review and Optimize Signals
- Review and Optimize Component Communication
- Fixing Error Handling / Server Authority 401/403
- Fixing bugs ;-)
- ...

## Bonus

A module for using Ratatui is under client-ratatui

## Links

Leptos Documentation:

https://book.leptos.dev/

For playing with Tailwind:

https://play.tailwindcss.com/

![alt text](docs/screenshot-leptos.png "screenshot Leptos")

![alt text](docs/screenshot-ratatui.png "screenshot Ratatui")