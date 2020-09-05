clean:
    cargo clean

run:
    cargo run

build:
    cargo build

brun: build
    ./target/debug/zipup



gadd:
    git add .

gcommit MESSAGE: gadd
    git commit -m "{{MESSAGE}}"