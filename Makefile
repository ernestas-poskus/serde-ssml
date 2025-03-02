.PHONY: release_minor
release_minor:
	cargo test && cargo release minor --execute

.PHONY: release_patch
release_patch:
	cargo test && cargo release patch --execute
