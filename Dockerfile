FROM node:22 AS ui-builder
WORKDIR /usr/src/marending-dev/ui

COPY ui/package.json ui/package-lock.json ./

RUN npm install
COPY ./ui/ ./
RUN npm run build

FROM rust:1.81 AS chef 
RUN cargo install cargo-chef 
WORKDIR /usr/src/marending-dev/service

FROM chef AS planner
COPY ./service .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/marending-dev/service/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY ./service .
COPY --from=ui-builder /usr/src/marending-dev/ui/dist ./ui
RUN cargo build --release --bin marending-dev 

FROM debian:bookworm-slim AS runtime

WORKDIR /usr/src/app/
COPY --from=builder /usr/src/marending-dev/service/target/release/marending-dev /usr/src/app/
ENTRYPOINT ["/usr/src/app/marending-dev"]