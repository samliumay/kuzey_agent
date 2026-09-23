# Kuzey Agent

A terminal AI agent I'm building from scratch to learn Rust. Work in progress.

## Status

- Interactive setup: pick a provider and model, enter your API key (masked)
- Chat loop with conversation history (`/exit` to quit)
- Tool calling: the model can ask for tools, the agent runs them and sends the results back (up to 10 rounds per message)
- System prompt, set in `ProviderConfig`
- Providers:
  - **Google Gemini**: works, including tools
  - **Ollama**: works, including tools. Hand-written over HTTP with `reqwest`
  - **OpenAI, Anthropic**: stubbed, not implemented yet

## Tools

| Tool | What it does |
|---|---|
| `get_current_time` | Returns the local date and time |
| `read_file` | Reads a text file |
| `write_file` | Creates or overwrites a file, and creates missing folders |

> `write_file` doesn't ask for confirmation yet, so the model can overwrite any file the agent can reach.

## Run

```bash
git clone https://github.com/samliumay/kuzey_agent.git
cd kuzey_agent/kuzey_agent
cargo run
```

You need a [Rust toolchain](https://www.rust-lang.org/tools/install), plus either a Gemini API key or a running [Ollama](https://ollama.com) with a tool-capable model (the model name in `providers.rs` must match `ollama list`).

## Layout

```text
kuzey_agent/src/
├── main.rs          # CLI prompts + chat / tool loop
├── providers.rs     # shared config, message types, dispatch to a provider
├── providers/       # one file per provider (google, ollama, openai, anthropic)
├── tools.rs         # tool specs + dispatch to a tool
└── tools/           # one file per tool (time, read_file, write_file)
```

## Roadmap

- [x] Interactive CLI
- [x] Gemini provider
- [x] Chat loop with conversation history
- [x] Tool calling (time, read file, write file)
- [x] Ollama provider
- [x] System prompt
- [ ] Ask the user before `write_file` runs
- [ ] `edit_file` tool (string replace)
- [ ] OpenAI and Anthropic providers
- [ ] Better errors and tests
