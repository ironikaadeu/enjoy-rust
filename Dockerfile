FROM rust:1.75.0

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

# CMD ["myapp"]
