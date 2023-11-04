################
##### Builder
################
FROM ghcr.io/dattito/rust-alpine-mimalloc:1.73 as builder

ARG TARGETARCH

RUN case "$TARGETARCH" in \
        "arm64" | "aarch64") rustup target add aarch64-unknown-linux-musl ;; \
        "amd64" | "x86_64")  rustup target add x86_64-unknown-linux-musl ;; \
        *)                   echo "Unsupported architecture: $TARGETARCH" && exit 1 ;; \
    esac

WORKDIR /app

COPY . .

RUN case "$TARGETARCH" in \
        "arm64" | "aarch64") cargo build --target aarch64-unknown-linux-musl --release ;; \
        "amd64" | "x86_64")  cargo build --target x86_64-unknown-linux-musl --release ;; \
        *)                   echo "Unsupported architecture: $TARGETARCH" && exit 1 ;; \
    esac

################
##### Runtime
################
FROM scratch AS runtime 

COPY --from=builder /app/target/*-unknown-linux-musl/release/timetable-dhbw-rust /

EXPOSE 3000

ENTRYPOINT ["/timetable-dhbw-rust"]
