# TODO

## Pre-commit hook setup
- [x] Create `.pre-commit-config.yaml` with hooks for:
  - [x] `cargo fmt --check`
  - [x] `cargo clippy -- -D warnings`
  - [x] `eslint` on staged JS/TS files (best-effort if eslint is missing)
- [ ] Run `pre-commit install` (requires installing the `pre-commit` Python package)
- [ ] Run `pre-commit run --all-files`


