#Builder stage
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

# Create app directory
WORKDIR /app

# Install lld and mold
RUN apt-get update && apt-get install clang mold -y


FROM chef as planner

# Copy source code
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

#Force SQLX to read metadata
ENV SQLX_OFFLINE=true

# Build in releace mode
RUN cargo build --release --bin harmony_server


#Runtime stage
FROM  debian:bookworm-slim AS runtime

# Create app directory
WORKDIR /app

# Install OpenSSL, Ca-certificates && clean up
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*

# Copy source code
COPY --from=builder /app/target/release/harmony_server harmony_server

# Copy configuration
COPY configuration configuration

# Set environment
ENV APP_ENVIRONMENT production

# Set entrypoint of container
ENTRYPOINT ["./harmony_server"]
