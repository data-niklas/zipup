clean:
    cargo clean

run *ARGS:
    cargo run -- {{ARGS}}

build:
    cargo build

release:
    cargo build --release

install:
    cp ./target/release/zipup $SCRIPTS/bin/

brun *ARGS: build
    ./target/debug/zipup {{ARGS}}

crun *ARGS:
    ./target/debug/zipup {{ARGS}}

rinstall: release install

test backup:
    cargo run -- --backup ./test/zipup.conf



gadd:
    git add .

gcommit MESSAGE: gadd
    git commit -m "{{MESSAGE}}"