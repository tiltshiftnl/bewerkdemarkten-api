FROM ekidd/rust-musl-builder:nightly-2020-11-19 as builder

COPY . /home/rust/
RUN cargo build --release

# Copy the statically-linked binary into a apline container (cannot use scratch, we need ca-certificates)
FROM scratch
# RUN apk update && apk add ca-certificates && rm -rf /var/cache/apk/*

VOLUME /tmp
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/bewerkdemarkten-api .
# USER 1000
EXPOSE 8000
CMD ["./bewerkdemarkten-api"]