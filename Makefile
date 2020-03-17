.DEFAULT_GOAL := help

# Porcelain
# ###############
.PHONY: run ci build lint test init

run: target/debug ## run the app
	./target/debug/countdown-time-rs

ci: setup lint test build ## run all tests and build all artifacts
	@echo "Not implemented"; false

todo: ## list all TODOs in the project
	grep --color=always -Ri todo .

build: setup target/release ## create artifact
	

lint: setup ## run static analysis
	@echo "Not implemented"; false

test: setup ## run all tests
	@echo "Not implemented"; false

init: ## one time setup
	direnv allow .

# Plumbing
# ###############
.PHONY: setup
setup: # noop

target/debug: src
	cargo build

target/release: src
	cargo build --release

# Utilities
# ###############
.PHONY: help
help: ## print this message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
