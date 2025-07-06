# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bookworm as builder

# Install required tools
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang

# Install cargo-leptos
# RUN cargo install cargo-leptos

# temporary fix, as naming of wasm-opt has been changed
RUN cargo install --locked --force cargo-leptos --git https://github.com/saikatdas0790/cargo-leptos --branch saikatdas0790/fix-macos-aarch64-arm64-mismatch

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
ENV DOMAIN=nicolas-frey.com
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/microweb /app/microweb

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="microweb"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_ROOT="pkg"

EXPOSE 3000

# Run the server
CMD ["/app/microweb"]
