# Start with a minimal base image
FROM debian:bullseye-slim

# Set environment variables to configure rustup and Cargo
ENV USER=root
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH="$CARGO_HOME/bin:$PATH"

# Install dependencies for rustup and rustc
RUN apt-get update && \
    apt-get install -y curl build-essential && \
    rm -rf /var/lib/apt/lists/*

# Download and install rustup (Rust's installer and version manager)
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Install Rust stable by default
RUN $CARGO_HOME/bin/rustup default stable

# Create a directory for your Rust projects
WORKDIR /usr/src/myapp