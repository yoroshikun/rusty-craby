#! /bin/bash

docker run -v cargo-cache:/root/.cargo/registry -v "$PWD:/volume" --rm -it clux/muslrust cargo build --release