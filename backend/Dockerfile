FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

# Use a lightweight image for production
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/microblog /app/microblog

RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

# Expose the port the Axum app listens on
EXPOSE 3000

# Run the app
CMD ["./microblog"]