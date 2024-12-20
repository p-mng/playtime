# playtime 🎮⏱️

A simple, cross-platform, privacy-oriented time tracker for the command line.

## Description

`playtime` is a command line tool that launches an executable, tracks how long the application stays open, and saves the session info (start timestamp and duration) to a database when the application is terminated. This is mainly intended for logging playtime in individual games (similar to what Steam or Epic Games offer), but can also be used as a simple way of tracking work or studying time (similar to [bartib](https://github.com/nikolassv/bartib) or [Timewarrior](https://github.com/GothenburgBitFactory/timewarrior)).

## Features

- **cross-platform**: works on Linux, macOS, Windows
- **simple**: human-readable config file using [TOML](https://toml.io/en/)
- **lightweight**: command line interface, no GUI, minimal dependencies
- **privacy-friendly**: no external services required, everything stays on your device

## Installation

- Download the Rust programming language: <https://www.rust-lang.org/tools/install>
- Install using `cargo install --git https://github.com/p-mng/playtime.git --tag v0.1.1`
