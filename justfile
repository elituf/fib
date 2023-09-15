set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# build fib
build:
    cargo build --release

# install or update fib to default cargo location
install:
    cargo install --path .
