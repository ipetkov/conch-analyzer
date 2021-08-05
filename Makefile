all: webpack

.PHONY: clean cleanall

wasm:
	wasm-pack build

webpack: wasm
	npm run-script --prefix ./www build

clean:
	cargo clean
	rm -rf ./pkg ./www/dist

cleanall: clean
	rm -rf ./www/node_modules
