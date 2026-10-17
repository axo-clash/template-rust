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

The bot's decision-making logic is implemented in `src/services/bot_service.rs`. The current implementation returns a fixed `SHOOT_UP` placeholder. Implement your own strategy in `play`.


## Pièges et zone rouge

Les modèles acceptent `PLACE_TRAP` avec `coordinates` et exposent `board[y][x].trap`
(`placedBy`, `placedAtTurn`, `armedFromTurn`, `expiresAfterTurn`) et
`bot.data.nextTrapTurn` (10 initialement). Le point d’entrée de stratégie retourne seulement `SHOOT_UP` comme réponse d’exemple.

Une pose remplace le piège présent sur la case. Posé au tour T, il est actif de
T+1 à T+10 inclus ; la recharge autorise une nouvelle pose à T+10. Après chaque
action, les pièges sous les deux bots infligent 1 PV puis disparaissent, y compris
sous leur créateur. Jouer deuxième peut donc empêcher de s’en échapper à temps.
La couleur du créateur est purement visuelle. Les pièges ne bloquent pas les tirs.
`currentTurn` compte les tours terminés : la prochaine action résout `currentTurn+1`.

La zone rouge est annoncée aux tours 70/80/90/100 et active aux tours 75/85/95/105,
du bord vers le centre : 1 PV par bot en zone active en fin de tour global.


## Écrire ta stratégie

Le template fournit les DTO et les endpoints HTTP. `BotService::play` retourne un
`PlayResponseDTO` complet ; le contrôleur le sérialise directement. Il ne contient
aucun calcul de zone, de recharge, d’armement, de simulation ou de choix tactique.
Ces comportements sont à implémenter dans ton bot à partir des données reçues.

Une réponse de piège se construit avec les DTO :

```rust
PlayResponseDTO {
    action: Action::PlaceTrap,
    coordinates: Some(CoordDTO { x: Some(3), y: Some(4) }),
}
```

Les tests vérifient uniquement le décodage et la sérialisation du contrat, y compris
les anciennes entrées sans piège. Pour tester le serveur démarré :

```sh
axo-clash check-bot -url http://127.0.0.1:9000/api/v1/bots -repeat 5
```

Le nouveau CLI effectue 435 vérifications (87 par passage).

## Architecture Docker pour Scaleway

Le Dockerfile utilise des stages `linux/amd64` et compile explicitement la cible
Rust `x86_64-unknown-linux-musl`. Le binaire copié dans l’image est donc un ELF
x86-64 statique, même lorsque le build est lancé sur un Mac ARM.
Le build refuse toute autre plateforme : `--platform linux/amd64` est obligatoire
sur un hôte ARM pour que les métadonnées de l’image correspondent aussi au binaire.

```sh
docker buildx build --platform linux/amd64 --load -t mon-bot:local .
docker image inspect mon-bot:local --format '{{.Os}}/{{.Architecture}}'
```

La commande d’inspection doit afficher `linux/amd64`. Pour publier, remplace
`--load` par `--push` et utilise le nom complet de ton image publique.
