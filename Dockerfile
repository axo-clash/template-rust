# --- Etape 1 : Build de l'application (binaire statique musl) ---
FROM rust:alpine AS build
WORKDIR /app

# Copie les manifests et pre-compile les dependances (cache Docker)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src target/release/deps/template_rust* target/release/template-rust

COPY . .
RUN touch src/main.rs && cargo build --release

# --- Etape 2 : Image finale minimale (FROM scratch) ---
# Le binaire est statiquement lie (musl), strippe et sans deps externes :
# aucune librairie ni certificat n'est necessaire au runtime.
FROM scratch

# Port d'ecoute par defaut (surchargeable via la variable d'environnement PORT)
ENV PORT=9000
EXPOSE 9000

COPY --from=build /app/target/release/template-rust /template-rust

ENTRYPOINT ["/template-rust"]
