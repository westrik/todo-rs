# todo-boilerplate

A simple SPA web-app built with Rust, Postgres, TypeScript, and React.

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
