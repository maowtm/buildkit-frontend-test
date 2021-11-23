FROM rust AS build
WORKDIR /usr/src/app
COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .
RUN cargo build --release

FROM debian
COPY --from=build /usr/src/app/target/release/bktest /
ENTRYPOINT ["/bktest"]
