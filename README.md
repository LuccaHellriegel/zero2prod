# Setup

Install rustup, cargo

```bash
# linker
sudo apt-get install lld clang

cargo install cargo-watch
cargo install cargo-tarpaulin
cargo install cargo-audit
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
