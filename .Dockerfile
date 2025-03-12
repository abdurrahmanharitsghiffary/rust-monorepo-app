FROM rust:1.82.0

WORKDIR /actix_starter

RUN cargo install sqlx-cli

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY ./src ./src
COPY ./migrations ./migrations
COPY .sqlx .sqlx

ENV SQLX_OFFLINE=true

RUN rm -f ./target/release/basic_crud*

RUN cargo build --release

CMD ["/basic_crud/target/release/rust_crud_basic"]