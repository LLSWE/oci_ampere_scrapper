FROM rust:slim as builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

FROM python:3.11-slim-bookworm

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN pip install oci-cli --break-system-packages

WORKDIR /app

COPY --from=builder /app/target/release/oci_cron /app/oci_cron

CMD ["./oci_cron"]
