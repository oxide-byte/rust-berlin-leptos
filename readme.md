# Rust - Berlin Clone

## Purpose

This implementation is a clone implementation of the Rust UserGroup 
**Rust Hack and Learn** in Berlin of the implementation: https://github.com/andreasklostermaier/halreslib (Original).

Link to the group: https://berline.rs/

## Implementation

The Frameworks in use:

* Leptos 7.5
* Tailwind 4.0

## Installation / Preparation

* Rust: https://www.rust-lang.org/tools/install
* Trunk: https://trunkrs.dev/guide/getting-started/installation.html
* TailwindCss: https://tailwindcss.com/docs/installation/using-vite

PS: For TailwindCss I use the Mac Brew option:

```shell
brew update
brew install tailwindcss
```

## Running

The implementation uses Trunk to generate and run the Client Side Pages. It includes Hot Reloading

```bash
trunk serve --open
```

## Build

```bash
trunk build --release
```

If you like to publish it on GitHub, don't forget to fix manually the base URL and links in index.html of the generated files.

https://oxide-byte.github.io/rust-berlin-leptos/

## Links

Leptos Documentation:

https://book.leptos.dev/

For playing with Tailwind:

https://play.tailwindcss.com/