FROM rust:1.48 as builder
WORKDIR /usr/src

# Rocket requires nightly
RUN rustup default nightly
# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new bewerkdemarkten-api
WORKDIR /usr/src/bewerkdemarkten-api
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application.
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=builder /usr/local/cargo/bin/bewerkdemarkten-api .
USER 1000
EXPOSE 8000
CMD ["./bewerkdemarkten-api"]