################
##### Builder
################
FROM ghcr.io/dattito/rust-alpine-mimalloc:1.73 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

################
##### Runtime
################
FROM scratch AS runtime 

COPY --from=builder /app/target/*-unknown-linux-musl/release/timetable-dhbw-rust /

EXPOSE 3000

ENTRYPOINT ["/timetable-dhbw-rust"]
