FROM rust:bookworm AS chef 
RUN cargo install cargo-chef 
WORKDIR /usr/src/marending-dev/service

FROM node:23-bookworm AS ui-builder
WORKDIR /usr/src/marending-dev/ui
RUN apt update && apt install -y python3 libsdl-pango-dev brotli gzip
ENV TZ="Europe/Zurich"

COPY ui/package.json ui/package-lock.json ./

RUN npm install
COPY ./ui/ ./
RUN npm run build
# Compress static assets
RUN find dist -type f \( -name "*.html" -o -name "*.js" -o -name "*.css" -o -name "*.json" -o -name "*.svg" \) \
  -exec gzip -9 -k {} \; \
  -exec brotli -q 11 -k {} \;

FROM chef AS planner
COPY ./service .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/marending-dev/service/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY ./service .
RUN cargo build --release --bin marending-dev 

FROM debian:bookworm-slim AS runtime

WORKDIR /usr/src/app/
COPY --from=builder /usr/src/marending-dev/service/target/release/marending-dev /usr/src/app/
COPY --from=ui-builder /usr/src/marending-dev/ui/dist ./ui
ENTRYPOINT ["/usr/src/app/marending-dev"]