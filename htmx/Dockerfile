FROM rust:1.77

WORKDIR /app

RUN rustup update nightly && \
    rustup default nightly && \
    cargo install cargo-watch

COPY . .

WORKDIR /app/htmx

EXPOSE 8080

CMD ["cargo", "watch", "-x", "run --bin htmx"]
