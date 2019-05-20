# FROM rust:1.34.2-stretch
FROM rustlang/rust:nightly

RUN mkdir /app

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo install --path .

COPY . ./

CMD ["cargo", "run"]
