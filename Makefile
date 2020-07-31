.PHONY: install-tools

install-tools:
	cargo install watchexec
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
