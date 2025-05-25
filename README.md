# scryfall-rs

A simple Rust crate for interacting with Scryfall's Magic: The Gathering API.

Notably, this crate adds support for all async runtimes, as opposed to the `scryfall` crate which only supports `tokio`. (TODO!!!!!!!)

Additionally, this crate also implements a centralized client system, meaning the API is properly ratelimited by default.
