# Build image
FROM ekidd/rust-musl-builder:nightly-2020-11-19 as builder

LABEL maintainer="Milo van der Linden <milo@@tiltshift.nl>"

COPY . /home/rust/
RUN cargo build --release

# Runtime image
FROM scratch

LABEL maintainer="Milo van der Linden <milo@@tiltshift.nl>"

# Repository used to get the JSON source files
ENV GIT_REPOSITORY=https://github.com/tiltshiftnl/makkelijke-markt-pakjekraam.git
# Replace the secret in a public facing API
ENV ROCKET_SECRET_KEY=8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=
# Database connection
ENV DATABASE_URL="postgresql://postgres:postgres@127.0.0.1:5432/bewerkdemarkten"

VOLUME /tmp
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/bewerkdemarkten-api .
EXPOSE 8000
CMD ["./bewerkdemarkten-api"]