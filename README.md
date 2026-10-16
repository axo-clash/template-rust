# Template Rust Bot

A Rust-based bot template for a game competition system. This project implements a web API server that provides endpoints for bot interaction in a turn-based game.

## Prerequisites

- [Rust](https://www.rustup.rs/) (latest stable version)
- Cargo (comes with Rust installation)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd template-rust
```

2. Install dependencies:
```bash
cargo build
```

## Running the Project

Start the server:
```bash
cargo run
```

The server will start and listen on `http://0.0.0.0:9000`

The listen port can be overridden with the `PORT` environment variable:
```bash
PORT=8080 cargo run
```

For development with automatic recompilation, you can use:
```bash
cargo watch -x run
```
(requires `cargo install cargo-watch`)

### Docker

Build the image:
```bash
docker build -t axoclash-bot-rust .
```

Run the container:
```bash
docker run -p 9000:9000 axoclash-bot-rust
```

The container listens on port 9000 by default. Override it with the `PORT` environment variable (and map the port accordingly):
```bash
docker run -e PORT=8080 -p 8080:8080 axoclash-bot-rust
```

The final image is built `FROM scratch`: it contains only the statically-linked binary (no OS, no libraries, no certificates — the bot does not call any external API).

## API Endpoints

- `GET /api/v1/bots/info` - Returns bot information
- `POST /api/v1/bots/play` - Accepts game state and returns bot action

## Development

### Quick Commands
```bash
cargo check    # Fast compilation check
cargo clippy   # Linting
cargo fmt      # Code formatting
cargo test     # Run tests (if any)
```

### Building for Production
```bash
cargo build --release
```

The release build is highly optimized with LTO, single codegen unit, and debug symbols stripped for maximum performance.

## Bot Logic

The bot's decision-making logic is implemented in `src/services/bot_service.rs`. The current implementation randomly selects actions as a starting template. Modify the `play` function to implement your game strategy.
