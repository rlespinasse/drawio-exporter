dev-local: fmt clippy-fix idioms-fix fix audit build test

build:
	@echo + $@
	cargo build

test:
	@echo + $@
	cargo test

fmt:
	@echo + $@
	cargo fmt --all

fmt-check:
	@echo + $@
	cargo fmt --all -- --check

clippy:
	@echo + $@
	cargo clippy --all-features --all-targets

clippy-fix:
	@echo + $@
	cargo clippy --fix --all-features --all-targets --allow-dirty --allow-staged

fix:
	@echo + $@
	cargo fix --allow-dirty --allow-staged

idioms-fix:
	@echo + $@
	cargo fix --edition-idioms --allow-dirty --allow-staged

update:
	@echo + $@
	cargo update

audit:
	@echo + $@
	cargo audit

DEPS=
deps:
	@echo + $@
	cargo tree $(DEPS)

release-%:
	@echo + $@
	cargo release --execute $*

dryrun-release-%:
	@echo + $@
	cargo release $*
