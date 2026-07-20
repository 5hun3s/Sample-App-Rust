FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

# 必要パッケージ
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libxdo-dev \
    file \
    && rm -rf /var/lib/apt/lists/*

# Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
 && apt-get install -y nodejs

# Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

# Tauri CLI
RUN cargo install create-tauri-app{
    "name": "tauri-dev",

    "build": {
        "dockerfile": "Dockerfile"
    },

    "customizations": {
        "vscode": {
            "extensions": [
                "rust-lang.rust-analyzer",
                "tauri-apps.tauri-vscode",
                "esbenp.prettier-vscode",
                "dbaeumer.vscode-eslint",
                "bradlc.vscode-tailwindcss"
            ]
        }
    },

    "forwardPorts": [
        1420,
        5173
    ],

    "postCreateCommand": "npm install -g npm"
}