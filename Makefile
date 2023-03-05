all:
	cargo build --release

	wasm-opt -Oz target/wasm32-unknown-unknown/release/raycaster.wasm \
    -o target/wasm32-unknown-unknown/release/raycaster.wasm

size: all
	du -b target/wasm32-unknown-unknown/release/raycaster.wasm

run: all
	w4 run-native target/wasm32-unknown-unknown/release/raycaster.wasm

bundle: all
	w4 bundle target/wasm32-unknown-unknown/release/raycaster.wasm --title "Raycaster" --html dist/raycaster.html

deploy: bundle
	npx gh-pages -d dist
