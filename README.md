# ![Linkr](static/logo.svg)

A modern self-hosted URL Shortener for both individuals and organizations that’s easy to set up and use.

Currently this project is still in its early stages. It is a fully functional URL shortener, but many quality of life features are still to come.

## Deployment

### To Heroku

Just click here and follow the instructions on screen to deploy:

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy?template=https://github.com/liamrosenfeld/linkr/tree/master)

To have logins persist between app refreshes, create an [environment variable](https://heroku.com/deploy?template=https://github.com/liamrosenfeld/linkr/tree/master) with the name `ROCKET_SECRET_KEY` and then run `openssl rand -base64 32` in a terminal and paste the result in the value.

### Elsewhere

You can deploy with docker to a wide range of hosts as long a you provide these environment variables at runtime:

- `PORT` (Most often passed by the host)
- `DATABASE_URL` (For the postgres database. Incudes login info for database)
- `ROCKET_SECRET_KEY` (Used for signing private cookies. Generate it yourself with `openssl rand -base64 32`)

## Local Development

### With Cargo

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. [Install Postgres](https://www.postgresql.org/download/)
3. Install the Diesel CLI: `cargo install diesel_cli`
4. Open terminal at project directory
5. Create a `.env` file that has the contents `DATABASE_URL=postgres://[YOUR USERNAME]:[YOUR PASSWORD]@localhost/linkr`
6. Use Rust nightly for this directory: `rustup override set nightly`
7. Setup the database: `diesel setup`
8. Run: `cargo run`

### With Docker

1. [Install Docker](https://docs.docker.com/get-docker/)
2. Run: `docker-compose up`

### New Database Migrations

After creating a new migration either run `diesel print-schema > './src/schema.rs'` or `diesel migration run`
