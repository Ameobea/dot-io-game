cd game-engine \
  && ./release.sh \
  && wasm-gc target/wasm32-unknown-unknown/release/game_engine.wasm \
  && wasm-bindgen ./target/wasm32-unknown-unknown/release/game_engine.wasm --out-dir ./build
yarn build || npm build
