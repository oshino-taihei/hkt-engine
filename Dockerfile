FROM rust:1.44-alpine

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["cargo", "run"]