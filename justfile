set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# build in release mode
build:
    cargo build --release

# install or update to default cargo location
install:
    cargo install --path .

# lints excessively with clippy
lint:
    @cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used
