# Kuzey Agent

A terminal AI agent I'm building from scratch to learn Rust. Work in progress.

## Status

- Interactive setup: pick a provider and model, enter your API key (masked)
- Google Gemini works: send a prompt, get a response
- Ollama, OpenAI and Anthropic are stubbed, not implemented yet

## Run

```bash
git clone https://github.com/samliumay/kuzey_agent.git
cd kuzey_agent/kuzey_agent
cargo run
```

You need a [Rust toolchain](https://www.rust-lang.org/tools/install) and a Gemini API key.

## Layout

```text
kuzey_agent/src/
├── main.rs          # CLI prompts
├── providers.rs     # shared config + dispatch to a provider
└── providers/       # one file per provider (google, ollama, openai, anthropic)
```

## Roadmap

- [x] Interactive CLI
- [x] Gemini provider
- [ ] Chat loop with conversation history
- [ ] Remaining providers
- [ ] Tool calling
- [ ] Better errors and tests
