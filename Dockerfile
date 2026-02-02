FROM rust:1.93-bullseye as builder

ARG PKG_NAME="vault-kms-provider"
ARG BIN_NAME="server"
ARG TARGET="x86_64-unknown-linux-musl"
ARG TARGETPLATFORM

ENV ARM_64="linux/arm64"
ENV AMD_64="linux/amd64"
ENV ARM_64_TARGET="aarch64-unknown-linux-musl"
ENV AMD_64_TARGET="x86_64-unknown-linux-musl"
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV CC_aarch64_unknown_linux_musl=clang
ENV AR_aarch64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUNNER="qemu-aarch64 -L /usr/aarch64-linux-gnu"

RUN apt-get update
RUN apt-get install protobuf-compiler musl-dev build-essential musl-tools clang llvm -y

RUN if [ "$TARGETPLATFORM" = "$ARM_64" ]; then rustup target add $ARM_64_TARGET; fi
RUN if [ "$TARGETPLATFORM" = "$AMD_64" ]; then rustup target add $AMD_64_TARGET; fi
RUN if [ "$TARGETPLATFORM" != "$ARM_64" ] && [ "$TARGETPLATFORM" != "$AMD_64" ]; then rustup target add $TARGET; fi

RUN mkdir /usr/src/$PKG_NAME
RUN mkdir /run/sockets

WORKDIR /usr/src/$PKG_NAME

COPY Cargo.toml Cargo.lock build.rs ./
COPY ./benches ./benches
COPY ./proto ./proto
COPY ./src ./src

RUN if [ "$TARGETPLATFORM" = "$ARM_64" ]; then cargo build --release --target=$ARM_64_TARGET && mv /usr/src/$PKG_NAME/target/$ARM_64_TARGET/release/$BIN_NAME /usr/src/$PKG_NAME/target/release/$BIN_NAME; fi
RUN if [ "$TARGETPLATFORM" = "$AMD_64" ]; then cargo build --release --target=$AMD_64_TARGET && mv /usr/src/$PKG_NAME/target/$AMD_64_TARGET/release/$BIN_NAME /usr/src/$PKG_NAME/target/release/$BIN_NAME; fi
RUN if [ "$TARGETPLATFORM" != "$ARM_64" ] && [ "$TARGETPLATFORM" != "$AMD_64" ]; then cargo build --release --target=$TARGET && mv /usr/src/$PKG_NAME/target/$TARGET/release/$BIN_NAME /usr/src/$PKG_NAME/target/release/$BIN_NAME; fi

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM scratch
LABEL authors="ruddickmg"
ARG UID=10001
ARG GID=10001
ARG PKG_NAME="vault-kms-provider"
ARG BIN_NAME="server"
WORKDIR /user/local/bin/
COPY --from=0 /etc/passwd /etc/passwd
COPY --from=builder /usr/src/$PKG_NAME/target/release/$BIN_NAME ./app
COPY --from=builder /run/sockets /run/sockets
USER $UID:$GID
EXPOSE 8080

CMD ["./app"]
