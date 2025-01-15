.PHONY: cargo

prerequisites: ## Install the Cargo requirements for testing
	@echo Installing Cargo plugins
	@cargo install cbindgen
	@cargo install cargo-nextest
	@cargo install cargo-llvm-cov
	@cargo install grcov

test: prerequisites ## Test the cargo project
	@cargo llvm-cov --lcov --output-path target/lcov.info nextest

cov: ## Test the cargo project with coverage reporting to stdout
	@cargo llvm-cov nextest

build: ## Build the cargo project
	@cargo build

build-release: ## Build the release version of the cargo project
	@cargo build --release

bump-dependencies: ## Install required bump dependencies
	@$(PYTHON) -m pip install --upgrade pip
	@pip install bump2version --user

bump-%: bump-dependencies ## Bump the (major, minor, patch) version of the application
	@bumpversion $*