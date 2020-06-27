# Linkr

A modern self-hosted URL Shortener for both individuals and organizations that’s easy to setup and use

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
