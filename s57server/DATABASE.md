# Database

We will be using the [Diesel](http://diesel.rs) ORM and [Postgis](https://postgis.net/) . 

## Running db for local dev
```bash
docker run --name s57server -p 127.0.0.1:5432:5432 -e POSTGRES_USER=admin -e POSTGRES_PASSWORD=mysecretpassword -e POSTGRES_DB=s57server -d postgis/postgis:13-3.1
```

## Diesel Info
see [diesel getting started guide](http://diesel.rs/guides/getting-started/)

Install CLI
```bash
cargo install diesel_cli --no-default-features --features postgres 
```

Create env variables
```
echo DATABASE_URL=postgres://admin:mysecretpassword@localhost/s57server > .env
```

## Workflow
- `diesel migration generate <table_name>`
- edit migrations/<timestamp>_<table_name> up.sql and down.sql
- `diesel migration run` or `diesel setup`

