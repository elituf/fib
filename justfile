set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# build fib
build:
    cargo build --release

# install or update fib to default cargo location
install:
    cargo install --path .

# lints excessively with clippy
lint:
    @cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used
