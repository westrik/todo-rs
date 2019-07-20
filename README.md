# todo-boilerplate-rs

[![Build Status](https://dev.azure.com/m0493/todo-rs/_apis/build/status/westrik.todo-boilerplate-rs?branchName=master)](https://dev.azure.com/m0493/todo-rs/_build/latest?definitionId=1&branchName=master)

A boilerplate web API server built with Rust and Postgres.

## Environment Setup

```sh
git clone --recurse-submodules git@github.com:westrik/todo-boilerplate-rs.git
ln -sf ../../.git_hooks/pre-push .git/hooks/pre-push
```

## Database Setup

```sh
createdb todo_boilerplate
createuser toodles
psql postgres
postgres=# alter user toodles with encrypted password 'pw';
postgres=# grant all privileges on database todo_boilerplate to toodles;
cat "DATABASE_URL=postgres://toodles:pw@localhost/todo_boilerplate" > .env
```

## Running the web server

