.PHONY: dev
.PHONY: serve
.PHONY: install-tools

RUST_WASM_CRATE:=client

install-tools:
	cargo install watchexec
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh


dev:
	watchexec -w client -r make serve

serve: www/pkg
	cd www && python3 -m http.server

www/pkg: ${RUST_WASM_CRATE}/*
	wasm-pack build --dev -t web -d ../www/pkg ${RUST_WASM_CRATE}
