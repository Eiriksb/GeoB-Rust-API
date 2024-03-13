# Use the official Rust image as the base image
FROM rust:1.76 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Create a new image from the smaller Alpine Linux image
FROM alpine:latest

# Set the working directory
WORKDIR /usr/src/app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/geojson-api .

# Expose the port your application will run on
EXPOSE 8081

# Run your application
CMD ["./geojson-api"]
