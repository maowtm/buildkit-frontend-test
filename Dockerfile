FROM rust AS build
WORKDIR /usr/src/app
RUN mkdir src && echo "fn main() {}" > src/main.rs
COPY ./Cargo.lock .
COPY ./Cargo.toml .
RUN cargo build --release
COPY ./src ./src
RUN cargo build --release

FROM debian
COPY --from=build /usr/src/app/target/release/bktest /
ENTRYPOINT ["/bktest"]
