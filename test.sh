#! /bin/bash

cargo test --features local
cargo test --features redis
cargo test --features nats
cargo test
