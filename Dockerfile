FROM rust:1.82-slim AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /app/target/release/seed-rust /usr/local/bin/seed-rust
ENV PORT=8080
EXPOSE 8080
CMD ["seed-rust"]
