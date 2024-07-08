# Use the official Rust image as the base image
FROM rust:1.76 as builder

# Create a new binary project
RUN USER=root cargo new --bin rust_web_mongo
WORKDIR /rust_web_mongo

# Copy the Cargo.toml and Cargo.lock files into the container
COPY ./Cargo.toml ./Cargo.lock ./

# Cache the dependencies - this step ensures that your dependencies
# are cached unless changes to one of the two Cargo files are made.
RUN cargo fetch --locked

# Copy the source code of your application into the container
COPY ./src ./src

# Build your application in release mode
# RUN cargo build --release
RUN cargo build --release


# Use Debian bullseye-slim as the runtime base image
FROM debian:bookworm-slim

# Install OpenSSL - required by Actix Web
RUN apt-get update \
    && apt-get install -y\
    openssl \
    libssl3 \
    gcc \
    ca-certificates \
    build-essential \
    libffi-dev \
    bc \
    sysstat \
    curl \
    iputils-ping \ 
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage to the runtime stage
COPY --from=builder /rust_web_mongo/target/release/rust_web_mongo /usr/local/bin/

# Set the working directory
WORKDIR /app

# Copy the static folder to the working directory in the Docker image 
# /app is working dir, the command is called under app -> /static need to be und /app
COPY ./static /app/static

# Expose the port on which your server will run
EXPOSE 18000

# Command to run the binary
CMD ["/usr/local/bin/rust_web_mongo"]
