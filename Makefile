all:
	cargo build --release

	wasm-opt -Oz target/wasm32-unknown-unknown/release/raycaster.wasm \
    -o target/wasm32-unknown-unknown/release/raycaster.wasm

size: all
	du -b target/wasm32-unknown-unknown/release/raycaster.wasm

run: all
	w4 run-native target/wasm32-unknown-unknown/release/raycaster.wasm

bundle: all
	w4 bundle target/wasm32-unknown-unknown/release/raycaster.wasm --title "Raycaster" --html dist/index.html

# Remember to check when https://github.com/tschaub/gh-pages/issues/354 is closed
deploy: bundle
	npx gh-pages@3.0.0 -d dist
