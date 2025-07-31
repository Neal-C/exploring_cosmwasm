# I take no responsibility for bloated Docker images :)

### wasmd ###
FROM cosmwasm/wasmd:v0.18.0 AS wasmd

### rust-optimizer ###
FROM cosmwasm/rust-optimizer:0.11.5 AS rust-optimizer

FROM rust

COPY --from=wasmd /usr/bin/wasmd /usr/local/bin/wasmd
COPY --from=wasmd /opt/* /opt/

RUN apt update \
    && apt install -y jq \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY . .

CMD cargo unit-test && cargo integration-test


