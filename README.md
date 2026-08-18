<div align="center">

# Kuzey Agent

**A personal AI agent built from scratch while learning Rust.**

![Rust](https://img.shields.io/badge/Rust-2024_edition-000000?style=flat-square&logo=rust)
![Status](https://img.shields.io/badge/status-work_in_progress-f59e0b?style=flat-square)

</div>

## About

Kuzey Agent is a learning project focused on exploring Rust by building a small,
terminal-based AI agent. The long-term goal is to understand the pieces behind
modern coding agents—from interactive command handling to model communication
and tool use—while keeping the implementation approachable.

> [!NOTE]
> This project is in its earliest stage. It is a place to experiment, learn, and
> improve one small piece at a time.

## Current Features

- Interactive command-line loop
- Basic user input handling
- `/exit` command
- Initial module structure for an Ollama client

## Getting Started

### Prerequisites

- A recent [Rust toolchain](https://www.rust-lang.org/tools/install)

### Run locally

```bash
git clone https://github.com/samliumay/kuzey_agent.git
cd kuzey_agent/kuzey_agent
cargo run
```

Type `/exit` to close the agent.

## Project Structure

```text
.
├── README.md
└── kuzey_agent/
    ├── Cargo.toml
    └── src/
        ├── main.rs
        └── clients/
            ├── mod.rs
            └── ollama_client.rs
```

## Roadmap

- [x] Create an interactive CLI
- [x] Add basic command handling
- [ ] Connect to Ollama
- [ ] Maintain conversation history
- [ ] Add a tool system
- [ ] Improve errors, logging, and tests
- [ ] Document lessons learned along the way

## Why This Project?

The best way to learn a language is to build something with it. Kuzey Agent is
my way of learning Rust through a practical project involving APIs, asynchronous
code, state management, error handling, and thoughtful CLI design.

Suggestions and learning resources are always welcome.

---

<div align="center">
Made with curiosity and Rust 🦀
</div>
