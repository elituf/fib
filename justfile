set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# build fib
build:
    cargo build --release

# install fib to default cargo location
install:
    cargo install --path .

# update the previously installed fib
update:
    cargo install --path . --force
