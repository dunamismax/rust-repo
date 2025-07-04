<p align="center">
  <img src="https://github.com/rust-lang/rust-artwork/blob/master/logo/rust-logo-512x512.png" alt="Rust Monorepo logo" width="250"/>
</p>

Welcome to my Rust Monorepo. This repository centralizes diverse Rust applications and libraries, designed for efficient, scalable, and maintainable project management using Cargo Workspaces.

[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/dunamismax/rust-repo/blob/main/LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://github.com/dunamismax/rust-repo/pulls)
[![GitHub Stars](https://img.shields.io/github/stars/dunamismax/rust-repo)](https://github.com/dunamismax/rust-repo/stargazers)

---

## Overview

This monorepo serves as a centralized hub for various Rust projects. It leverages Cargo's workspace feature to manage multiple crates (packages) within a single repository, ensuring consistent dependency management and simplified build processes.

---

## Repository Structure

```sh
rust-repo/
├── apps/
│   ├── blog-cli/
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   ├── http-server/
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── weather-cli/
│       ├── .env.example
│       ├── Cargo.toml
│       └── src/main.rs
│
├── libs/
│   └── markdown-generator/
│       ├── Cargo.toml
│       └── src/lib.rs
│
├── tools/
│   └── link-checker/
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
git clone https://github.com/dunamismax/rust-repo.git
cd rust-repo

# 2. Build all projects in the workspace
cargo build

# 3. Run an application (e.g., the http-server)
cargo run -p http-server
```

---

## Projects Overview

This monorepo hosts several applications, libraries, and tools.

### Applications

*   **[HTTP Server](https://github.com/dunamismax/rust-repo/blob/main/apps/http-server/src/main.rs)**: A lightweight, asynchronous HTTP server built with **Hyper** and **Tokio**. It demonstrates handling basic GET and POST requests, serving a simple HTML landing page and a JSON API endpoint.
*   **[Weather CLI](https://github.com/dunamismax/rust-repo/blob/main/apps/weather-cli/src/main.rs)**: A command-line weather application that fetches and displays the current weather for a specified city. It uses the OpenWeatherMap API and requires an API key to be set in a `.env` file.
*   **[Blog Post Generator](https://github.com/dunamismax/rust-repo/blob/main/apps/blog-cli/src/main.rs)**: A command-line application for generating simple markdown blog posts from a template. It leverages the `markdown-generator` library to create the post content.

### Libraries

*   **[Markdown Generator](https://github.com/dunamismax/rust-repo/blob/main/libs/markdown-generator/src/lib.rs)**: A reusable library for creating markdown content. It provides a `Post` struct and a function to generate a formatted markdown string from post data.

### Tools

*   **[Link Checker](https://github.com/dunamismax/rust-repo/blob/main/tools/link-checker/src/main.rs)**: A developer tool that scans markdown files and checks for broken or invalid URLs, helping to ensure the quality of documentation and blog posts.

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
# cp apps/weather-cli/.env.example .env
cargo run -p weather-cli -- London

# Generate a new blog post
cargo run -p blog-cli -- --title "My First Post" --author "dunamismax"

# Check for broken links in a markdown file
cargo run -p link-checker -- "path/to/your/post.md"
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