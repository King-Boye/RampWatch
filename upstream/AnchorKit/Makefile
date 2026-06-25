.PHONY: build test lint fmt clean deploy-testnet help

## build: Build the contract in release mode
build:
	cargo build --release

## test: Run all tests
test:
	cargo test

## lint: Run clippy with warnings as errors
lint:
	cargo clippy -- -D warnings

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
