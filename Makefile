.PHONY: serve

RUST_WASM_CRATE:=client

serve: www/pkg
	cd www && python3 -m http.server

www/pkg: ${RUST_WASM_CRATE}
	wasm-pack build -t web -d ../www/pkg ${RUST_WASM_CRATE}
