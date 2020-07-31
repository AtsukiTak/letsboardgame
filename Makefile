.PHONY: dev
.PHONY: serve
.PHONY: install-tools

install-tools:
	cargo install watchexec
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh


dev:
	watchexec -w . -r make serve

serve: www/pkg
	cd www && python3 -m http.server

www/pkg: src/* Cargo.toml Cargo.lock
	cargo build && wasm-pack build --dev -t web -d ./www/pkg
