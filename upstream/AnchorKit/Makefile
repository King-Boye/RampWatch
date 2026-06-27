.PHONY: build test lint fmt clean deploy-testnet help build-wasm storybook validate coverage

## build: Build the contract in release mode
build:
	cargo build --release

## test: Run all Rust tests
test:
	cargo test

## lint: Run clippy with warnings as errors
lint:
	cargo clippy -- -D warnings

## build-wasm: Build the contract WASM artifact used for deployment
build-wasm:
	cargo build --release --target wasm32-unknown-unknown

## storybook: Run Storybook (UI components)
storybook:
	cd ui && npm run storybook

## validate: Run repo validation (configs + Rust validation)
validate:
	./validate_all.sh

## coverage: Run Rust + UI coverage (best-effort)
coverage:
	cargo tarpaulin --version >/dev/null 2>&1 || true
	@echo "(1/2) Rust coverage"
	@cargo tarpaulin --ignore-tests 2>/dev/null || cargo test
	@echo "(2/2) UI coverage"
	cd ui && npm run test:coverage

## fmt: Format code with rustfmt
fmt:
	cargo fmt

## clean: Remove build artifacts
clean:
	cargo clean

## deploy-testnet: Deploy contract to Stellar testnet
deploy-testnet:
	soroban contract deploy \
		--wasm target/wasm32-unknown-unknown/release/anchor_kit.wasm \
		--network testnet

## help: Show this help message
help:
	@grep -E '^## ' Makefile | sed 's/## //'

