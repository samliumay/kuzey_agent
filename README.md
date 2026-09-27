# Kuzey Agent

A terminal AI agent written from scratch in Rust. You chat with a model, and the model can use
tools on your machine (read and write files, run shell commands) to get things done.

It talks to Gemini, Claude and local Ollama models through the same chat and tool loop. No agent
framework is involved: every provider is a hand-written translation between Kuzey's own message
types and that API's JSON.

> **Learning project.** I'm building this to learn Rust, so the code is commented with notes and
> open questions. It works, but it's not hardened. Read the [safety](#safety) section before
> using it.

## What it can do

- Pick a provider and model interactively; API keys are typed in masked and never stored
- Multi-turn chat with full conversation history
- **Tool calling**: the model asks for a tool, Kuzey runs it and feeds the result back, for up
  to 10 rounds per message, so it can chain steps (read a file → edit it → run a command to
  check it)

### Providers

| Provider | Status | Default model | Needs |
|---|---|---|---|
| Google Gemini | ✅ chat + tools | `models/gemini-3.8-flash` | [Gemini API key](https://aistudio.google.com/apikey) |
| Anthropic Claude | ✅ chat + tools | `claude-haiku-4-5-20251001` | [Anthropic API key](https://console.anthropic.com/settings/keys) |
| Ollama (local) | ✅ chat + tools | `qwen3.8:latest` | [Ollama](https://ollama.com) running on `localhost:11434` |
| OpenAI | 🚧 not implemented yet | `gpt-5.6-astra` | |

### Tools

| Tool | What it does |
|---|---|
| `get_current_time` | Returns the local date and time |
| `read_file` | Reads a text file |
| `write_file` | Creates or overwrites a file, creating missing folders |
| `run_command` | Runs a shell command in a given folder, returns exit code + stdout + stderr (30 s timeout) |

## Quick start

**Requirements:** Rust **1.85 or newer** (the crate uses edition 2024). Install it with
[rustup](https://rustup.rs). Check with `cargo --version`.

```bash
git clone https://github.com/samliumay/kuzey_agent.git
cd kuzey_agent/kuzey_agent
cargo run --release
```

Then answer the prompts:

```text
Kuzey Agent 0.1.0
> Which provider are you going with? google
> Please enter your API key: ********
> Which model do you want to use? models/gemini-3.8-flash
> You: what time is it, and what's in Cargo.toml?
[tool] get_current_time {}
[tool] read_file {"path":"Cargo.toml"}
<the model's answer>
> You: /exit
```

Lines starting with `[tool]` show every tool the model runs, with its arguments.
Type `/exit` to quit.

### Using a local model (Ollama)

No API key needed. Start Ollama and pull a model that supports tool calling:

```bash
ollama pull qwen3.8
cargo run --release     # choose "ollama"
```

The model name must match `ollama list`. To use a different one, change it in
`ProviderKind::models()` in `src/providers.rs`.

## Safety

The model decides which tools to call, and Kuzey runs them **without asking you first**:

- `write_file` can overwrite any file your user account can write to.
- `run_command` can run any shell command, including destructive ones.

A model can get this wrong by itself, or be steered by text inside a file it reads. Until
confirmation prompts land (see the [roadmap](#roadmap)), run Kuzey in a folder you don't mind
changing, ideally inside a VM or container, and watch the `[tool]` lines.

`run_command` uses `sh`, so it works on Linux and macOS but not on Windows yet.

## How it works

```text
            ┌───────────────────────────────────────────────┐
 You: ...   │ history: Vec<ChatMessage>                     │
 ─────────► │   User · Assistant · ToolCalls · ToolResult   │
            └──────────────────────┬────────────────────────┘
                                   │ providers::send()
                    ┌──────────────▼──────────────┐
                    │ google.rs / anthropic.rs /  │  translate history + tool specs
                    │ ollama.rs                   │  into that API's JSON, call it
                    └──────────────┬──────────────┘
                                   │ Reply
               ┌───────────────────┴───────────────────┐
         Reply::Text                          Reply::ToolCalls
   print it, add to history,         run each with tools::execute(),
   wait for your next message        add calls + results to history,
                                     call the provider again (max 10×)
```

Each provider owns its translation: Gemini wants `Part`s and a thought signature echoed
back, Anthropic wants `tool_use` / `tool_result` blocks matched by id, Ollama uses
OpenAI-style `tool_calls`. `main.rs` never knows which one it's talking to.

## Project layout

```text
kuzey_agent/src/
├── main.rs          # CLI prompts + the chat / tool loop
├── providers.rs     # ProviderConfig, ChatMessage, Reply, dispatch to a provider
├── providers/       # google.rs · anthropic.rs · ollama.rs · openai.rs (stub)
├── tools.rs         # ToolSpec, ToolCall, dispatch to a tool
└── tools/           # time.rs · read_file.rs · write_file.rs · run_command.rs
```

### Adding a tool

1. Create `src/tools/my_tool.rs` with two functions:
   - `spec() -> ToolSpec`: name, description and a JSON Schema for the arguments. This is
     all the model sees, so the description matters.
   - `async fn run(args: &Value) -> String`: do the work, return the result (or an
     `"Error: ..."` string, so the model can react instead of the agent crashing).
2. In `src/tools.rs`: add `mod my_tool;`, put `my_tool::spec()` in `specs()`, and add a
   `"my_tool" => my_tool::run(&call.args).await` arm in `execute()`.

Every provider picks it up automatically.

## Roadmap

- [x] Interactive CLI with masked API key input
- [x] Chat loop with conversation history
- [x] Tool calling with multi-step rounds
- [x] Gemini, Ollama and Anthropic providers
- [x] Tools: time, read file, write file, run command
- [ ] Ask before `write_file` and `run_command` run
- [ ] OpenAI provider
- [ ] `edit_file` tool (find and replace instead of rewriting whole files)
- [ ] Windows support for `run_command` (`cmd /C`)
- [ ] Streaming responses
- [ ] Better errors and tests
