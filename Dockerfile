FROM rust AS builder
WORKDIR /opt/app

COPY Cargo.toml Cargo.lock /opt/app/
COPY src/ /opt/app/src/
RUN cargo build --release

FROM debian
RUN apt-get update && apt-get install -y openssl curl
COPY --from=builder /opt/app/target/release/medat-wf-bot /opt/bin/
CMD /opt/bin/medat-wf-bot
