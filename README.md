# Rust Pratice 1

## Usage

### Database

.env

```
DATABASE_URL=postgres://postgres:postgres@localhost/todo
```

    sqlx database create
    sqlx migrate run

### App

    cargo run --bin api
    # cargo run --bin api_diesel

## Benchmark

Using [k6](https://github.com/grafana/k6).

    k6 run --vus 100 --iterations 10000 benchmark.js
