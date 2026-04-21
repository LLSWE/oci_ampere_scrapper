FROM rust:slim

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev ca-certificates \
    python3 python3-pip curl \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install oci-cli --break-system-packages
# RUN curl -L https://raw.githubusercontent.com/oracle/oci-cli/master/scripts/install/install.sh -o install.sh \
#     && chmod +x install.sh \
#     && ./install.sh --accept-all-defaults --exec-site-path /usr/local/bin \
#     && rm install.sh
#
WORKDIR /app
COPY . .
RUN cargo build --release

CMD ["./target/release/oci_cron"]
