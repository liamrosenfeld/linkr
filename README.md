# ![Linkr](static/logo.svg)

A modern self-hosted URL Shortener for both individuals and organizations that’s easy to set up and use.

Built with Rust (using Rocket) for the backend and Svelte for the frontend.

Currently this project is still in its early stages. It is a fully functional URL shortener, but many quality of life features are still to come.

## Deployment

> Note: Make sure you set up HTTPS (user passwords are transmitted)

### To Heroku

Just click here and follow the instructions on screen to deploy:

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy?template=https://github.com/liamrosenfeld/linkr/tree/master)

To have user login sessions persist between server restarts, create an [environment variable](https://devcenter.heroku.com/articles/config-vars) with the name `ROCKET_SECRET_KEY` and then run `openssl rand -base64 32` in a terminal and paste the result in the value.

> Note: If using Heroku with Cloudflare use the Heroku app url, not the one they tell you to use when setting up a DNS

### Elsewhere

You can deploy with docker to a wide range of hosts as long a you provide these environment variables at runtime:

- `PORT` (Most often passed by the host)
- `DATABASE_URL` (For the postgres database. Incudes login info for database)
- `ROCKET_SECRET_KEY` (Used for signing private cookies. Generate it by running `openssl rand -base64 32` in a terminal)

## Setup

1. Go to the page `/setup` to create your first account before you publicly release the website
2. Go to `/new_user` to create accounts for the rest of your team

## Local Development

Steps to run on localhost:8000

### Fully Local

Backend:

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. [Install Postgres](https://www.postgresql.org/download/)
   - Start with `pg_ctl -D /usr/local/var/postgres start`
3. Install the Diesel CLI: `cargo install diesel_cli`
4. Open terminal at project directory
5. Create a `.env` file that has the contents `DATABASE_URL=postgres://[YOUR USERNAME]:[YOUR PASSWORD]@localhost/linkr`
6. Setup the database: `diesel setup`
7. Run: `cargo run`

Frontend:

1. Install [npm](https://www.npmjs.com/)
2. Navigate to the `frontend` directory
3. Run `npm install`
4. Run `npm run build`

### With Docker

1. [Install Docker](https://docs.docker.com/get-docker/)
2. Run: `docker compose up`

### New Database Migrations

After creating a new migration either run `diesel print-schema > './src/schema.rs'` or `diesel migration run`
