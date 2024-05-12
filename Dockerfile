### BUILD STAGE
FROM messense/rust-musl-cross:x86_64-musl as builder

WORKDIR /app

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl


### FINAL BINARY STAGE
FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/aa_server /

ENTRYPOINT ["/aa_server"]

#Just documentation, should match application PORT used as env variable for exposing serverin the docker container
EXPOSE 3001