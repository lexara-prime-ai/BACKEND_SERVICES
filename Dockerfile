FROM rust:latest

WORKDIR /app/

COPY . .

RUN rustup default

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

EXPOSE 8000

CMD ["cargo", "watch", "--why", "--", "echo"]

