<p align="center">
  <img src="https://www.rust-lang.org/static/images/rust-logo-512x512.png" alt="Rust Monorepo logo" width="250"/>
</p>

Welcome to my Rust Monorepo. This repository centralizes diverse Rust applications and libraries, designed for efficient, scalable, and maintainable project management using Cargo Workspaces.

[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/sawyer/rust-repo/blob/main/LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://github.com/sawyer/rust-repo/pulls)
[![GitHub Stars](https://img.shields.io/github/stars/sawyer/rust-repo?style=social)](https://github.com/sawyer/rust-repo/stargazers)

---

## Overview

This monorepo serves as a centralized hub for various Rust projects. It leverages Cargo's workspace feature to manage multiple crates (packages) within a single repository, ensuring consistent dependency management and simplified build processes.

---

## Repository Structure

```sh
rust-repo/
├── apps/
│   ├── http-server/
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── weather-cli/
│       ├── .env.example
│       ├── Cargo.toml
│       └── src/main.rs
│
├── libs/
│   └── placeholder-lib/
│       ├── Cargo.toml
│       └── src/lib.rs
│
├── tools/
│   └── placeholder-tool/
│       ├── Cargo.toml
│       └── src/main.rs
│
├── .gitignore
├── Cargo.toml          # Root workspace configuration
├── LICENSE
└── README.md
```

---

## Quick Start

To get started, clone the repository, build the projects, and run an application.

```bash
# 1. Clone the repository
git clone https://github.com/sawyer/rust-repo.git
cd rust-repo

# 2. Build all projects in the workspace
cargo build

# 3. Run an application (e.g., the http-server)
cargo run -p http-server
```

---

## Projects Overview

This monorepo hosts several applications and libraries.

### [HTTP Server](apps/http-server/src/main.rs)

A lightweight, asynchronous HTTP server built with **Hyper** and **Tokio**. It demonstrates handling basic GET and POST requests, serving a simple HTML landing page and a JSON API endpoint.

### [Weather CLI](apps/weather-cli/src/main.rs)

A command-line weather application that fetches and displays the current weather for a specified city. It uses the OpenWeatherMap API and requires an API key to be set in a `.env` file.

---

## How to Compile and Run

This project uses **Cargo**, the Rust build tool and package manager, to compile and run all crates in the workspace.

### Build All Crates

To build all applications and libraries, run the `cargo build` command from the root of the repository. For an optimized release build, use the `--release` flag.

```bash
# Build for development
cargo build

# Build for production
cargo build --release
```

### Run Applications

Once built, you can run the applications using `cargo run -p <package_name>`.

```bash
# Run the HTTP server
# It will be available at http://127.0.0.1:3000
cargo run -p http-server

# Run the weather CLI
# First, copy .env.example to .env and add your API key
# cp apps/weather-cli/.env.example apps/weather-cli/.env
cargo run -p weather-cli -- London
```

### Clean Build Artifacts

To remove all compiled files and build artifacts, run:

```bash
cargo clean
```

---

## Contributing

Contributions are welcome! Please feel free to fork the repository, create a feature branch, and open a pull request.

---

## Connect

Connect with the author, **dunamismax**, on:

- **Twitter:** [@dunamismax](https://twitter.com/dunamismax)
- **Bluesky:** [@dunamismax.bsky.social](https://bsky.app/profile/dunamismax.bsky.social)
- **Reddit:** [u/dunamismax](https://www.reddit.com/user/dunamismax)
- **Discord:** `dunamismax`
- **Signal:** `dunamismax.66`

---

## License

This repository is licensed under the **MIT License**. See the `LICENSE` file for more details.
