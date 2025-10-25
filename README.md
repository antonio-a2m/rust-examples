Práctica 1: Consumir json local con reqwest

docker-compose run rust-app bash
cargo new test-clase
cd test-clase
cargo add serde
cargo add serde_json

ejemplo1 a main

Practica 2
docker compose run rust-app bash
cargo new ejemplo2
cd ejemplo2
cargo add reqwest --features blocking
cp ../ejemplo2.rs src/main.rs
cargo run
