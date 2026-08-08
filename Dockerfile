FROM node:23-bookworm AS ui-builder
WORKDIR /usr/src/marending-dev/ui
RUN apt update && apt install -y python3 libsdl-pango-dev brotli gzip
ENV TZ="Europe/Zurich"

COPY ui/package.json ui/package-lock.json ./

RUN --mount=type=cache,target=/root/.npm \
    --mount=type=cache,target=/usr/src/marending-dev/ui/node_modules \
    npm install
COPY ./ui/ ./
RUN --mount=type=cache,target=/usr/src/marending-dev/ui/node_modules \
    npm run build

# Compress static assets
RUN find dist -type f \( -name "*.html" -o -name "*.js" -o -name "*.css" -o -name "*.json" -o -name "*.svg" -o -name "*.xml" -o -name "*.txt" \) \
  -exec gzip -9 -k {} \; \
  -exec brotli -q 11 -k {} \;

FROM rust:bookworm AS builder
WORKDIR /usr/src/marending-dev/service
COPY ./service .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/src/marending-dev/service/target \
    cargo build --release --bin marending-dev \
    && cp target/release/marending-dev /marending-dev

FROM debian:bookworm-slim AS runtime
WORKDIR /usr/src/app/
COPY --from=builder /marending-dev /usr/src/app/marending-dev
COPY --from=ui-builder /usr/src/marending-dev/ui/dist ./ui
ENTRYPOINT ["/usr/src/app/marending-dev"]
