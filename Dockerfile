from rust:1.41 as build

WORKDIR /usr/src/devctm
COPY ./ ./

RUN rustup set profile minimal
RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/devctm /build-out/

FROM ubuntu

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=build /build-out/devctm /
RUN mkdir -p /var/lib/lets-encrypt/persistence /var/lib/lets-encrypt/key_and_cert /var/lib/lets-encrypt/acme

RUN groupadd -g 999 devctm && useradd -r -u 999 -g devctm devctm
RUN chown -R devctm /var/lib/lets-encrypt && chmod 700 /var/lib/lets-encrypt
USER devctm

CMD ["/devctm"]
