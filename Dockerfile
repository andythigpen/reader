FROM rust:1.70 as chef

ARG TAILWIND_VERSION=v3.3.2

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-chef
RUN cargo install trunk
RUN wget -q -O /bin/tailwindcss https://github.com/tailwindlabs/tailwindcss/releases/download/$TAILWIND_VERSION/tailwindcss-linux-x64 && \
    chmod a+x /bin/tailwindcss
WORKDIR app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin server
RUN cd frontend && trunk build --release

FROM gcr.io/distroless/cc-debian11
COPY --from=builder /app/target/release/server /server
COPY --from=builder /app/dist/ /dist
CMD ["/server"]
