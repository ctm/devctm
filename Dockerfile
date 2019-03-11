from rust:1.33 as build

WORKDIR /usr/src/devctm
COPY ./ ./

RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/devctm /build-out/

FROM ubuntu

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=build /build-out/devctm /
RUN mkdir -p /usr/local/cargo/bin -p /ssl

RUN groupadd -g 999 devctm && useradd -r -u 999 -g devctm devctm
RUN mkdir -p /var/devctm && chown -R 999 /ssl /var/devctm && chmod 700 /var/devctm
USER devctm

EXPOSE 8088-8090
CMD ["/devctm"]
