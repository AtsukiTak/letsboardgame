.PHONY: dev
.PHONY: serve

dev:
	watchexec -w src -r make serve

serve: www/pkg
	cd www && python3 -m http.server

www/pkg: src/* Cargo.toml Cargo.lock
	cargo build && wasm-pack build --dev -t web -d ./www/pkg
