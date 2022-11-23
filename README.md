# Rust Pratice 1

## Installation

### Ubuntu

    sudo apt-get install libpq-dev

### Build

    cargo build --release

## Usage

### Database

.env

```
DATABASE_URL=postgres://postgres:postgres@localhost/todo
```

    sqlx database create
    sqlx migrate run

### Run API Server

    # Actix-web + sqlx
    cargo run --release --bin api

    # Actix-web + diesel
    cargo run --release --bin api_diesel

## Benchmark

Using [k6](https://github.com/grafana/k6).

    k6 run --vus 100 --iterations 10000 benchmark/create_todo.js
    k6 run --vus 100 --iterations 10000 benchmark/get_todo_list.js
    k6 run --vus 100 --iterations 10000 benchmark/get_todo.js
