.PHONY: help
.DEFAULT_GOAL := help

RUST_BACKTRACE=												1
CARGO_INCREMENTAL=										0
CARGO_REGISTRIES_CRATES_IO_PROTOCOL=	sparse

UNAME := $(shell uname)

ifeq ($(UNAME), Linux)
	TARGET_TRIPLE= x86_64-unknown-linux-gnu
endif
ifeq ($(UNAME), Darwin)
	TARGET_TRIPLE= x86_64-apple-darwin
endif

help:
	@awk 'BEGIN {FS = ":.*##"; printf "Usage: make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-25s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Development

install-mdbook-linkcheck: ## Install mdbook-linkcheck
ifeq (,$(wildcard ./doc/mdbook-linkcheck))
	@cd ./doc && \
	curl -Ls https://github.com/Michael-F-Bryan/mdbook-linkcheck/releases/latest/download/mdbook-linkcheck.${TARGET_TRIPLE}.zip -o mdbook-linkcheck.zip && \
  unzip -n mdbook-linkcheck.zip mdbook-linkcheck && \
  chmod +x mdbook-linkcheck && \
	rm mdbook-linkcheck.zip
endif

test: ## Run all tests
	cargo build -p lalrpop
	cargo test --all --all-features
	# Check the documentation examples separately so that the `lexer` feature
	# specified in tests do not leak into them
	cargo check -p calculator
	cargo check -p pascal
	cargo check -p whitespace

backup-lrgrammar: ## Create a backup of the lrgrammar.rs file
	@cp -f lalrpop/src/parser/lrgrammar.rs lalrpop/src/parser/lrgrammar.rs.bak

update-lrgrammar: backup-lrgrammar snapshot ## Update the generated lrgrammar.rs file

##@ Release

build-mdbook: install-mdbook-linkcheck ## Builds the LALRPOP tutorial (using `mdbook build`) and then copies the additional sources files
	@cd ./doc && \
	export PATH=$$PWD:$$PATH; mdbook build && \
	cp -rf calculator pascal whitespace book/html
	@echo "Book has been built into ${PWD}/book/html"
	@echo "To view, open ${PWD}/book/html/index.html"

snapshot: ## Generate lalprop snapshot of itself
	@cargo run -p lalrpop -- --force --no-whitespace --out-dir . lalrpop/src/parser/lrgrammar.lalrpop
