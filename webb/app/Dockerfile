# Rust as the base image
FROM rust:bullseye

RUN USER=root cargo new --bin webb
WORKDIR /webb

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

#RUN cargo build --release
#RUN rm src/*.rs

COPY ./migrations ./migrations
COPY ./src ./src

#RUN rm ./target/release/deps/webb*
#RUN cargo install --path .

#CMD ["webb"]

