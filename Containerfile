# --- frontend ---------------------------------------------------------------
# Node 26 is the current LTS. Odd-numbered releases (23, 25) reach end of life
# within months and their image tags stop moving, so they never pick up fixes for
# e.g. the brotli decoder bundled in Node.
FROM node:26-bookworm AS ui-builder
WORKDIR /usr/src/marending-dev/ui

RUN apt-get update \
    && apt-get install -y --no-install-recommends python3 libsdl-pango-dev brotli gzip \
    && rm -rf /var/lib/apt/lists/*
ENV TZ="Europe/Zurich"

COPY ui/package.json ui/package-lock.json ./
RUN --mount=type=cache,target=/root/.npm \
    --mount=type=cache,target=/usr/src/marending-dev/ui/node_modules \
    npm ci

COPY ./ui/ ./
RUN --mount=type=cache,target=/usr/src/marending-dev/ui/node_modules \
    npm run build

# Compress static assets. The service serves these precompressed variants, so
# the runtime image carries the plain files plus their .gz and .br siblings.
RUN find dist -type f \( -name "*.html" -o -name "*.js" -o -name "*.css" -o -name "*.json" -o -name "*.svg" -o -name "*.xml" -o -name "*.txt" \) \
    -exec gzip -9 -k {} \; \
    -exec brotli -q 11 -k {} \;

# --- backend ----------------------------------------------------------------
FROM rust:bookworm AS builder
WORKDIR /usr/src/marending-dev/service

COPY service/Cargo.toml service/Cargo.lock ./
# Prime the dependency cache with a throwaway binary, so a source-only change
# does not invalidate the dependency layer.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/src/marending-dev/service/target \
    mkdir -p src && echo 'fn main() {}' > src/main.rs \
    && cargo build --release \
    && rm -rf src

COPY service/src ./src
# COPY keeps the build context's mtimes, which are older than the fingerprints
# recorded by the priming build. Touch the sources so cargo rebuilds the real
# binary instead of leaving the throwaway one in place. The size check fails
# loudly if that ever happens.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/src/marending-dev/service/target \
    find src -exec touch {} + \
    && cargo build --release \
    && cp target/release/marending-dev /tmp/marending-dev \
    && test "$(stat -c %s /tmp/marending-dev)" -gt 1000000

# --- runtime ----------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

WORKDIR /usr/src/app
COPY --from=builder /tmp/marending-dev /usr/local/bin/marending-dev
COPY --from=ui-builder /usr/src/marending-dev/ui/dist ./ui

ENV SERVE_PORT=3000 \
    HOME=/tmp

# Numeric on purpose: the quadlet runs with keep-id, so uid 1000 is the
# deploying host user and no /etc/passwd entry is needed.
USER 1000:1000

EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/marending-dev"]
