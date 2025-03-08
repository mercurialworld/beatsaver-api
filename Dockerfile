FROM rust:1.85

WORKDIR /beatsaver
COPY . .

RUN cargo install --path .
