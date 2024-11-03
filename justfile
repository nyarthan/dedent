alias help := default

default:
    @just --list

format:
    treefmt

format-check:
    @treefmt --fail-on-change

lint:
    @cargo clippy

test:
    @cargo test

build:
    @cargo build --release
