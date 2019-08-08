# todo-rs
[![Build Status](https://dev.azure.com/m0493/todo-rs/_apis/build/status/westrik.todo-rs?branchName=master)](https://dev.azure.com/m0493/todo-rs/_build/latest?definitionId=2&branchName=master)

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
echo "ROCKET_DATABASES='{todo_db={url="postgres://toodles:pw@localhost/todo_app"}}'" > .env
echo "DATABASE_URL='postgres://toodles:pw@localhost/todo_app'" >> .env

```

## Running the web server

```sh
cargo run --bin app
```
