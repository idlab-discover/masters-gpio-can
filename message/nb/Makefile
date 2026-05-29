CROSS_TARGET ?= aarch64-unknown-linux-gnu
REMOTE_DIR ?= ~/can/nb
CRATES := $(filter-out target/,$(wildcard */))

.PHONY: $(MAKECMDGOALS)

$(MAKECMDGOALS):
	@set -e; \
	host="$@"; \
	cross build --target="$(CROSS_TARGET)" --release --workspace; \
	for crate in $(CRATES); do \
		crate="$${crate%/}"; \
		scp "target/$(CROSS_TARGET)/release/$$crate" "$$host:$(REMOTE_DIR)"; \
	done
