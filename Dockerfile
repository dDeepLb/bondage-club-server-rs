ARG RUST_VERSION=1.88.0
ARG APP_NAME=server
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

RUN \
    --mount=type=bind,source=common,target=common \
    --mount=type=bind,source=db,target=db \
    --mount=type=bind,source=network,target=network \
    --mount=type=bind,source=server,target=server \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/server
EOF

FROM debian:bullseye-slim AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser
COPY --from=build /bin/server /bin/
EXPOSE 4288
CMD ["/bin/server"]