FROM alpine:latest
COPY ./target/x86_64-unknown-linux-musl/release/rusty-craby /rusty-craby
ENTRYPOINT ["/rusty-craby"]