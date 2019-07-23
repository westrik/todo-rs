# todo-rs

Simple web API server built with Rust and Postgres.

## Environment Setup

```sh
git clone --recurse-submodules git@github.com:westrik/todo-rs.git
ln -sf ../../.git_hooks/pre-push .git/hooks/pre-push
```

## Database Setup

```sh
createdb todo_app
createuser toodles
psql postgres
postgres=# alter user toodles with encrypted password 'pw';
postgres=# grant all privileges on database todo_app to toodles;
cat "ROCKET_DATABASES='{todo_db={url="postgres://toodles:pw@localhost/todo_app"}}'" > .env

```

## Running the web server

```sh
cargo run --bin app
```
