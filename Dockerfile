from rust:1.33 as build

WORKDIR /usr/src/devctm
COPY ./ ./

RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/devctm /build-out/

FROM ubuntu

ENV DEBIAN_FRONTEND=noninteractive
COPY --from=build /build-out/devctm /

RUN groupadd -g 999 devctm && useradd -r -u 999 -g devctm devctm
USER devctm

EXPOSE 8088
CMD ["/devctm"]
