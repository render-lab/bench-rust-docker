# Seed: Rust service with a Dockerfile

Minimal axum HTTP service with `/` and `/health`, plus a multi-stage
`Dockerfile`. No deploy config.

```bash
cargo run
# or
docker build -t seed . && docker run -p 8080:8080 seed
```
