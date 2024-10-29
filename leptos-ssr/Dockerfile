FROM rustlang/rust:nightly-bullseye

WORKDIR /app

# Install Node.js from NodeSource repository
RUN apt-get update && apt-get install -y ca-certificates curl gnupg && \
    mkdir -p /etc/apt/keyrings && \
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg && \
    echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_20.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list && \
    apt-get update && apt-get install -y \
    pkg-config \
    nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-leptos
RUN cargo install cargo-leptos cargo-watch

# Install tailwindcss
RUN npm install -g tailwindcss

COPY . .

WORKDIR /app/leptos-ssr

EXPOSE 3333
EXPOSE 3334

CMD ["cargo", "leptos", "watch"]
