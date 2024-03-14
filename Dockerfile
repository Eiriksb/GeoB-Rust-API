FROM rust:1.76-slim-buster as builder
WORKDIR /app
COPY . .
RUN cargo install --path .
FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/geojson-api /usr/local/bin/geojson-api
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8080
CMD ["geojson-api"]