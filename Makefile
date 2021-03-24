build:
	@echo + $@
	cargo build

test:
	@echo + $@
	cargo test

fmt:
	@echo + $@
	cargo fmt --all -- --check

clippy:
	@echo + $@
	cargo clippy --all-features

DEPS=
deps:
	@echo + $@
	cargo tree $(DEPS)

release-%:
	@echo + $@
	cargo release -- $*

dryrun-release-%:
	@echo + $@
	cargo release --dry-run -vv -- $*
