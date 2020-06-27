FROM rustlang/rust:nightly

ENV ROCKET_ENV=production \
    ROCKET_ADDRESS=0.0.0.0

WORKDIR /
COPY . .

RUN cargo update
RUN cargo build --release
EXPOSE ${PORT}
CMD ROCKET_PORT="$PORT" target/release/linkr
