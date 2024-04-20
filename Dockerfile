# Latest stable release for Rust
FROM rust:1.77.2

# Create app directory
WORKDIR /app

# Install lld and mold
RUN apt update && apt install lld clang mold -y

# Copy source code
COPY . .

# Build in releace mode
RUN cargo build --release

# Set entrypoint of container
ENTRYPOINT ["./target/release/harmony_server"]
