# Build stage
FROM rust:1.93.1 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/bingo-now /usr/local/bin/bingo-now
EXPOSE 8080
CMD ["/usr/local/bin/bingo-now"]
