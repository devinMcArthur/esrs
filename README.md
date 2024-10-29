# Development

## Prerequisites

Need to use Rust nightly

```shell
rustup toolchain install nightly
rustup override set nightly
```

Add the wasm32-unknown-unknown target

```shell
rustup target add wasm32-unknown-unknown
```

Leptos [DX](https://book.leptos.dev/getting_started/leptos_dx.html)


## Docker Compose

To start the development environment, with all necessary resources, run the following command:

```shell
docker compose up
```

## Start Eventstore Locally

```shell
docker run --name esdb-node -it -p 2113:2113 \
    eventstore/eventstore:latest --insecure --run-projections=All \ 
    --enable-atom-pub-over-http
```
