
.PHONY=clean fixtures help test

VENV = .virtualenv/virtualenv.py
VENV_DIR := .pyenv-$(shell uname)
PYTHON := $(VENV_DIR)/bin/python
PIP := $(VENV_DIR)/bin/pip

help:
	$(info -- Targets -- )
	$(info )
	$(info fixtures -   rebuild all fixtures required by tests)
	$(info test     -   Run cargo-test, assuring fixtures are up-to-date)
	$(info clean    -   remove all intermediate and generated files (which are not checked in to git))
	$(info )

test: fixtures
	cargo test --test ser_unit

fixtures: tests/structs/fixed.rs tests/ser_data.rs



tests/structs/out.rs: tests/structs/in.rs Makefile
	@touch src/lib.rs
	@cargo build 2>/dev/null
	@echo "Generating data-structures at '$@'"

tests/structs/fixed.rs: tests/structs/out.rs
	@sed "s/derive_Default/derive(Default)/" $< > $@
	@echo "Writing fixed data-structures to '$@'"

$(VENV):
	wget -nv https://pypi.python.org/packages/source/v/virtualenv/virtualenv-12.0.7.tar.gz -O virtualenv-12.0.7.tar.gz
	tar -xzf virtualenv-12.0.7.tar.gz && mv virtualenv-12.0.7 ./.virtualenv && rm -f virtualenv-12.0.7.tar.gz
	chmod +x $@

$(PYTHON): $(VENV)
	$(VENV) -p python2.7 $(VENV_DIR)
	$(PIP) install git+https://github.com/Byron/bcore
	# $(PIP) install pyyaml

tests/ser_data.rs: tests/ser_gen.py $(PYTHON)
	$(PYTHON) $< > $@

clean:
	rm -Rf $(VENV_DIR)