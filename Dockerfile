FROM rustlang/rust:nightly-alpine as builder

# Install build dependencies
RUN apk add --no-cache musl-dev ca-certificates gcc g++ make

# Create a new empty project
WORKDIR /app
COPY . .

# Enable the 2024 edition
ENV RUSTFLAGS="-C target-feature=+crt-static -Z unstable-options"
RUN rustup component add rust-src --toolchain nightly
RUN RUSTFLAGS="-Z allow-features=edition2024" cargo +nightly build --release

# Create a minimal image from scratch
FROM scratch

# Copy the statically-linked binary
COPY --from=builder /app/target/release/ip_lookup /ip_lookup

# Copy SSL certificates from the builder
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

# Set environment variable for authentication
ENV ACCESS_TOKEN="default_token_change_me"

# Expose the port the app runs on
EXPOSE 3000

# Run the binary
ENTRYPOINT ["/ip_lookup"]
