# just manual: https://just.systems/man/en/

_default:
	@just --list

# Audits the app's dependencies for security vulnerabilities and unpermitted licenses
audit:
	cargo deny check advisories && cargo deny check licenses

# Lints the codebase (via clippy)
check:
	cargo clippy --locked

# Formats the codebase (via cargo fmt)
format:
	cargo fmt

# Builds and runs
run *args:
	cargo run --locked {{args}}

# Runs all tests
test *args:
	cargo test --locked {{args}}

# Lints and tests on every change
watch:
	cargo watch -w src -w Cargo.toml -c -x "clippy --locked" -x "test --locked"
