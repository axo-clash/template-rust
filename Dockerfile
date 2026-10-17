# Scaleway runs linux/amd64, including when this image is built on an ARM host.
FROM --platform=linux/amd64 rust:alpine AS build
WORKDIR /app
ARG TARGETPLATFORM
RUN test "$TARGETPLATFORM" = "linux/amd64" || { \
        echo 'Build for Scaleway with: docker buildx build --platform linux/amd64 ...' >&2; exit 1; \
    }
RUN apk add --no-cache build-base \
    && rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --locked --release --target x86_64-unknown-linux-musl \
    && rm -rf src target/x86_64-unknown-linux-musl/release/deps/template_rust* \
        target/x86_64-unknown-linux-musl/release/template-rust

COPY . .
RUN touch src/main.rs \
    && cargo build --locked --release --target x86_64-unknown-linux-musl

FROM --platform=linux/amd64 scratch
ENV PORT=9000
EXPOSE 9000
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/template-rust /template-rust
ENTRYPOINT ["/template-rust"]
