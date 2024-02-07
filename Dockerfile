FROM rust:1.75.0

WORKDIR /usr/src/myapp
COPY . .

RUN cd /usr/src/myapp && \
    rustc main.rs

CMD ["./main"]
