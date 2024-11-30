# Use the official Rust image as the base
FROM rust:1.73

# Install additional tools (e.g., Wasm target for Substrate or Node.js if needed)
RUN rustup target add wasm32-unknown-unknown \
    && apt-get update && apt-get install -y build-essential cmake pkg-config libssl-dev

# Set the working directory in the container
WORKDIR /usr/src/aoc2024

# Expose the cargo bin directory to PATH for easier development
ENV PATH="/usr/local/cargo/bin:${PATH}"

# Set up default command to keep the container running
CMD ["tail", "-f", "/dev/null"]
