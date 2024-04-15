FROM rust:1.76.0 AS builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

# Find and copy the libsqlite3.so.0 library
RUN find / -name libsqlite3.so.0 -exec cp {} /tmp/libsqlite3.so.0 \;


FROM gcr.io/distroless/cc-debian12
COPY --from=builder /usr/src/app/target/release/videas /usr/local/bin/videas
# Copy the libsqlite3.so.0 library to the final image
COPY --from=builder /tmp/libsqlite3.so.0 /usr/lib/

WORKDIR /app
# Copy required files to the final image
COPY --from=builder /usr/src/app/static ./static
COPY --from=builder /usr/src/app/templates ./templates
COPY --from=builder /usr/src/app/db.sqlite3 ./db.sqlite3


ENV DATABASE_URL="file:db.sqlite3"
ENV RUST_LOG=debug

EXPOSE 8080

CMD ["/usr/local/bin/videas"]