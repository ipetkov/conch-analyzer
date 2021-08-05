all: webpack

.PHONY: clean clean-cargo clean-dist clean-all gh-pages

wasm:
	wasm-pack build

webpack: wasm
	npm run-script --prefix ./www build

clean-cargo:
	cargo clean

clean-dist:
	rm -rf ./pkg ./www/dist

clean-node:
	rm -rf ./www/node_modules

clean: clean-cargo clean-dist

clean-all: clean clean-node

CUR_BRANCH := $(shell git branch --show-current)
gh-pages: webpack
	[[ -z $$(git status --short) ]] || (echo "cannot build with dirty directory"; false)
	cur_branch=$$(git branch --show-current)
	git branch -D gh-pages || true
	git switch --orphan gh-pages
	git add www/dist
	git mv www/dist/* .
	git commit -m 'Update gh-pages'
	git checkout ${CUR_BRANCH}
