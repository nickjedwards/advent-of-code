FROM rust:1.74

WORKDIR /usr/src/advent-of-code

COPY . .

RUN cargo install --path .

CMD ["cargo", "run"]
