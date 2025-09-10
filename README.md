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

The server will start and listen on `http://0.0.0.0:6000`

For development with automatic recompilation, you can use:
```bash
cargo watch -x run
```
(requires `cargo install cargo-watch`)

## API Endpoints

- `GET /info` - Returns bot information
- `POST /play` - Accepts game state and returns bot action

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