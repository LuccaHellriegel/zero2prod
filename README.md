# Setup

Install rustup, cargo

```bash
# linker
sudo apt-get install lld clang
sudo apt install postgresql-client

cargo install cargo-watch
cargo install cargo-tarpaulin
cargo install cargo-audit
cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls,postgres
```

# Commands

```bash
# coverage
cargo tarpaulin --ignore-tests
# watch
cargo watch -x check
cargo watch -x check -x test -x run
# lint
cargo clippy -- -D warnings
# format
cargo fmt
cargo fmt -- --check
# audit
cargo audit
```

# Migrations

```bash
export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
sqlx migrate add create_subscriptions_table

sqlx migrate run
```

# pgAdmin

https://www.pgadmin.org/download/pgadmin-4-apt/

```bash
curl -fsS https://www.pgadmin.org/static/packages_pgadmin_org.pub | sudo gpg --dearmor -o /usr/share/keyrings/packages-pgadmin-org.gpg
sudo sh -c 'echo "deb [signed-by=/usr/share/keyrings/packages-pgadmin-org.gpg] https://ftp.postgresql.org/pub/pgadmin/pgadmin4/apt/$(lsb_release -cs) pgadmin4 main" > /etc/apt/sources.list.d/pgadmin4.list && apt update'
sudo apt install pgadmin4
```
