FROM ekidd/rust-musl-builder:nightly-2020-11-19 as builder

COPY . /home/rust/
RUN cargo build --release

# Copy the statically-linked binary into a apline container (cannot use scratch, we need ca-certificates)
FROM scratch
# RUN apk update && apk add ca-certificates && rm -rf /var/cache/apk/*
ENV GIT_REPOSITORY=https://github.com/Amsterdam/fixxx-pakjekraam.git
ENV ROCKET_SECRET_KEY=8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=
VOLUME /tmp
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/bewerkdemarkten-api .

# USER 1000
EXPOSE 8000
CMD ["./bewerkdemarkten-api"]