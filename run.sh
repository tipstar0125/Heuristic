
cargo run -r --manifest-path tools/Cargo.toml --bin tester cargo run -r --features local --bin $1 < $2 > $3
#cargo run -r --manifest-path tools/Cargo.toml --bin tester cargo run -r --bin $1 < $2 > $3
