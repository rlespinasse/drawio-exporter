#!/usr/bin/env -S just --justfile

set quiet := true

default:
	just --choose

# Execute all developements recipes
[group('Development mode')]
loop: fmt clippy-fix idioms-fix fix audit build test

# Build it
[group('Development mode')]
build:
	cargo build

# Test it
[group('Development mode')]
test:
	cargo test

# Format it
[group('Development mode')]
fmt:
	cargo fmt --all

# Check the format
[group('CI/CD mode')]
fmt-check:
	cargo fmt --all -- --check

# Run linter on sources
[group('CI/CD mode')]
clippy:
	cargo clippy --all-features --all-targets

# Apply some lint suggestions from clippy
[group('Development mode')]
clippy-fix:
	cargo clippy --fix --all-features --all-targets --allow-dirty --allow-staged

# Apply rustc's suggestions from diagnostics like warnings and apply them to your source code
[group('Development mode')]
fix:
	cargo fix --allow-dirty --allow-staged

# Apply suggested idioms for the current edition
[group('Development mode')]
idioms-fix:
	cargo fix --edition-idioms --allow-dirty --allow-staged

# Update dependencies
[group('Development mode')]
update:
	cargo update

# Audit the code
[group('Development mode')]
audit:
	cargo audit

# Analyse dependencies tree
[group('Debug mode')]
deps DEPS="drawio-exporter":
	cargo tree --package {{ DEPS }}

# Release a new version (Possible values: major, minor, patch, release, rc, beta, alpha)
[group('Release mode')]
release LEVEL="alpha":
	cargo release --execute {{ LEVEL }}

# Dry-run a release
[group('Release mode')]
dryrun-release LEVEL="alpha":
	cargo release {{ LEVEL }}
