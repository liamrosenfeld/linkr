# ---- Build the Backend ---- 
FROM rust:1.57 as backend

WORKDIR /backend
COPY Cargo.lock .
COPY Cargo.toml .
COPY src src
COPY migrations migrations
COPY sqlx-data.json .

RUN cargo build --release

# ---- Build the Frontend ---- 
FROM node:17 as frontend

WORKDIR /frontend
COPY frontend .

RUN npm install && npm run build

# ---- Run ---- 
FROM debian:buster-slim as run

RUN apt-get update && apt-get install -y postgresql-client

ENV ROCKET_ENV=production \
    ROCKET_ADDRESS=0.0.0.0

COPY --from=backend /backend/target/release/linkr /usr/local/bin/linkr
COPY --from=frontend /frontend/build/ /static

EXPOSE ${PORT}
CMD ROCKET_PORT="$PORT" linkr
