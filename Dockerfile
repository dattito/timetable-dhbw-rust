ARG RUST_VERSION=1.76.0
ARG BINARY_NAME=timetable-dhbw-rust

FROM ghcr.io/dattito/rust-alpine-mimalloc:$RUST_VERSION AS chef 
RUN cargo install cargo-chef 
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM scratch AS runtime 
ARG BINARY_NAME
COPY --from=builder /app/target/*-unknown-linux-musl/release/$BINARY_NAME /app
EXPOSE 3000
ENTRYPOINT ["/app"]

