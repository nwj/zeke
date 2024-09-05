# just manual: https://just.systems/man/en/

_default:
	@just --list

# Audits the app's dependencies for security vulnerabilities and unpermitted licenses
audit:
	cargo deny check advisories && cargo deny check licenses

# Lints the codebase (via clippy)
check:
	cargo clippy

# Formats the codebase (via cargo fmt)
format:
	cargo fmt

# Builds and runs
run:
	cargo run

# Runs all tests
test:
	cargo test

# Lints and tests on every change
watch:
	cargo watch -w src -w Cargo.toml -c -x "clippy" -x "test"
