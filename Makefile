
.PHONY=clean fixtures help

help:
	$(info Targets)
	$(info fixtures       -   rebuild all fixtures required by tests)

fixtures: tests/structs/fixed.rs

tests/structs/out.rs: tests/structs/in.rs Makefile
	@touch src/lib.rs
	@cargo build 2>/dev/null
	@echo "Generating data-structures at '$@'"

tests/structs/fixed.rs: tests/structs/out.rs
	@sed "s/derive_Default/derive(Default)/" $< > $@
	@echo "Writing fixed data-structures to '$@'"