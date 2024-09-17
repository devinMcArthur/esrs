FROM rust:1.77

WORKDIR /app

RUN cargo install cargo-watch

COPY . .

EXPOSE 8080

CMD ["cargo", "run"]
