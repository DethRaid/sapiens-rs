# Unofficial Sapiens Rust API

This repo provides a Rust API, along with `cargo-generate` templates, which allow one to develop mods for the upcoming 
video game [Sapiens](https://store.steampowered.com/app/1060230/Sapiens/) using the Rust programming language

## Quickstart

* Install `cargo`
* Install [`cargo-generate`](https://github.com/ashleygwilliams/cargo-generate)
    * `cargo install cargo-generate --features vendored-openssl`
* Download the project template
    * `cargo generate --git https://github.com/DethRaid/sapiens-rust-mod-template.git`
* Edit the default features in `Cargo.toml` for the type of mod you're making
    * `biome` for a biome mod, `particles` for a particles mod, etc
* Write your mod
    * TODO: Wiki page about doing just that
* Build and upload your mod
    * TODO: Custom `cargo` command to build, package, and upload a mod
* Enjoy! 

## Overview
This repo has two main components: a Rust wrapper for the Sapiens API available to mods, and some `cargo-generate` 
project templates so that your mod can start with all relevant entry points already defined