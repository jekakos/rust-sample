FROM rust:1-slim-bookworm AS chef
RUN cargo install cargo-chef
RUN apt-get update -y && \
    apt-get install -y --no-install-recommends libssl-dev clang mold pkg-config openssh-client git && \
    apt-get autoremove -y && \
    apt-get autoclean -y && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json ./recipe.json
RUN mkdir -p -m 0600 ~/.ssh && \
    ########## NDA
    RUN  --mount=type=ssh cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/static-debian12:latest-amd64
COPY --from=curl /usr/bin/curl /bin/curl
COPY --from=curl /lib/ld-musl-x86_64.so.1 /lib/ld-musl-x86_64.so.1
COPY --from=curl /lib/libz.so.1 /lib/libz.so.1
COPY --from=curl /lib/libssl.so.3 /lib/libssl.so.3
COPY --from=curl /lib/libcrypto.so.3 /lib/libcrypto.so.3
COPY --from=curl /usr/lib/libcurl.so.4 /usr/lib/libcurl.so.4
COPY --from=curl /usr/lib/libnghttp2.so.14 /usr/lib/libnghttp2.so.14
COPY --from=curl /usr/lib/libidn2.so.0 /usr/lib/libidn2.so.0
COPY --from=curl /usr/lib/libssh2.so.1 /usr/lib/libssh2.so.1
COPY --from=curl /usr/lib/libbrotlidec.so.1 /usr/lib/libbrotlidec.so.1
COPY --from=curl /usr/lib/libunistring.so.5 /usr/lib/libunistring.so.5
COPY --from=curl /usr/lib/libbrotlicommon.so.1 /usr/lib/libbrotlicommon.so.1

FROM gcr.io/distroless/static-debian12:latest-amd64
COPY --from=chef /usr/bin/dash /bin/dash
COPY --from=chef /lib/x86_64-linux-gnu/libssl.so.3 /lib/x86_64-linux-gnu/libssl.so.3
COPY --from=chef /lib/x86_64-linux-gnu/libcrypto.so.3 /lib/x86_64-linux-gnu/libcrypto.so.3
COPY --from=chef /lib/x86_64-linux-gnu/libgcc_s.so.1 /lib/x86_64-linux-gnu/libgcc_s.so.1
COPY --from=chef /lib/x86_64-linux-gnu/libm.so.6 /lib/x86_64-linux-gnu/libm.so.6
COPY --from=chef /lib/x86_64-linux-gnu/libc.so.6 /lib/x86_64-linux-gnu/libc.so.6
COPY --from=chef /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2
COPY --from=builder /app/target/release/********** /
COPY --from=builder /app/resources /resources

CMD ["/service-service"]