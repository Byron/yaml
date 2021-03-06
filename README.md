YAML serialization and deserialization.
It follows the [YAML Spec 1.2][yaml-spec].

# Current State

[![Build Status](https://travis-ci.org/serde-rs/yaml.svg?branch=master)](https://travis-ci.org/serde-rs/yaml)
[![Latest Version](https://img.shields.io/crates/v/serde_yaml.svg)](https://crates.io/crates/serde_yaml)

**The project is so early in development that it cannot be used at all**.

## Test-Driven Development

All development is backed by at least one unit- or integration-test. To keep the source files as light 
and small as possible, unit-tests are put in place where integration tests would usually be. Taken that 
into consideration, we might end up not differentiating between these types too much after all.

When testing *serialization*, we will use *syntex* to generate the serialization code needed for our 
test-data structures. The desired output is generated by a python program which uses an existing pure-python
implementation of YAML for reference.

All dependencies between generated files are help within our top-level *Makefile*. It is made so that the default
target will print a help text with all available targets.

To run tests and assure all generated files are up-to-date, please run `make test`.

## Prerequisites

To run tests using the provided infrastructure, you will need the following:

* **make**
	- The make program deals with inter-file dependencies and runs the correct processes
* **wget**
	- is used to install the python virtual environment

Please note that the build system is not expected to work on Windows.

## Running Tests

As the repository contains all required files (even the generated ones), you are able to run tests without 
any prerequisites and on Windows using `cargo test`.

# Development Screencasts

All development is recorded and made available on *YouTube*. No cutting, no script, just 
my plain day, every day, coding with narration.

* **[Building a YAML Parser in Rust](https://www.youtube.com/playlist?list=PLMHbQxe1e9MmX_OeeyFLlqyZrBQ6PgGjL)** (*Playlist*)


# What is YAML

TODO

# Limitations

This paragraph contains the differences of the implementation compared to 
the requirements of the underlying [yaml specification][yaml-spec].

* NO SERIALZATION SUPPORT
* NO DESERIALIZTION SUPPORT

# Deviations from the Spec

This is an exhaustive list items showing where this implementation deviates from the recommendations of the 
[SPEC][yaml-spec].

* TODO ... 

[yaml-spec]: http://www.yaml.org/spec/1.2/spec.html
