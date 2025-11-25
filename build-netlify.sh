rustup default stable
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
mkdir -p deploy
cp ./target/wasm32-unknown-unknown/release/wordle.wasm ./deploy
cp index.html ./deploy
cp -r assets ./deploy
