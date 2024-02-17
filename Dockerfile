FROM rust:buster
WORKDIR /usr/src/app
COPY . .

RUN cargo install diesel_cli --no-default-features --features sqlite # ORM
RUN diesel setup # Migrations
RUN cargo build --release

CMD ["cargo", "run", "--release"]
