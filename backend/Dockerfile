# Stage 1: Prepare the dependency cache
FROM rust:latest AS chef

RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# Stage 3: Create a minimal runtime image
FROM gcr.io/distroless/cc AS runtime

WORKDIR /app
COPY --from=builder /app/target/release/microblog /app/microblog

EXPOSE 3000
CMD ["/app/microblog"]