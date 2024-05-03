# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.77.1
ARG APP_NAME=server

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies, a cache mount to /usr/local/cargo/git/db
# for git repository dependencies, and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=server/src,target=server/src \
    --mount=type=bind,source=server/Cargo.toml,target=server/Cargo.toml \
    --mount=type=bind,source=server/Cargo.lock,target=server/Cargo.lock \
    --mount=type=bind,source=shared/src,target=shared/src \
    --mount=type=bind,source=shared/Cargo.toml,target=shared/Cargo.toml \
    --mount=type=bind,source=shared/Cargo.lock,target=shared/Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release --manifest-path server/Cargo.toml && \
cp ./server/target/release/$APP_NAME /bin/server

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage. This uses the alpine image as the foundation for running the app.
FROM alpine:3.18 AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
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

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

EXPOSE 8123

# What the container should run when it is started.
CMD ["/bin/server"]