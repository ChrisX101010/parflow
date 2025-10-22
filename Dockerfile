# Multi-stage build for parflow-rest
FROM rust:1.72 as builder
WORKDIR /usr/src/parflow
COPY . .
RUN cargo build --release -p parflow-rest

FROM debian:stable-slim
COPY --from=builder /usr/src/parflow/target/release/parflow-rest /usr/local/bin/parflow-rest
EXPOSE 3000
CMD ["/usr/local/bin/parflow-rest"]
