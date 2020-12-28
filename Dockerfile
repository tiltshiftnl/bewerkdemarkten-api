# Build image
FROM ekidd/rust-musl-builder:nightly-2020-11-19 as builder
COPY . /home/rust/
RUN cargo build --release

# Runtime image
FROM scratch
# Repository used to get the JSON source files
ENV GIT_REPOSITORY=https://github.com/Amsterdam/fixxx-pakjekraam.git
# Replace the secret in a public facing API
ENV ROCKET_SECRET_KEY=8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=
VOLUME /tmp
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/bewerkdemarkten-api .
EXPOSE 8000
CMD ["./bewerkdemarkten-api"]