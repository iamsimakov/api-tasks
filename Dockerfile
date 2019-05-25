# FROM rust:1.34.2-stretch
FROM rustlang/rust:nightly

RUN mkdir /app

WORKDIR /app
# ADD src /app/src
# COPY Cargo.toml Cargo.lock ./
# RUN cargo install --path .

COPY . ./
EXPOSE 8000
CMD ["cargo", "run"]
