alias b := build
alias d := deploy

build:
    cargo build

test:
    cargo test

deploy:
    fly deploy