FROM rust:latest as builder
WORKDIR /usr/src/card-service
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/card-service /usr/local/bin/card-service
EXPOSE 8080
CMD ["card-service"]
