YAML serialization and deserialization.
It follows the [YAML Spec 1.2][yaml-spec].

# Current State

**The project is so early in development that it cannot be used at all**.

# What is YAML

TODO

# Limitations

This paragraph contains the differences of the implementation compared to 
the requirements of the underlying [yaml specification][yaml-spec].

* NO SERIALZATION SUPPORT
* NO DESERIALIZTION SUPPORT

# Build Instructions

The build is meant to work using the stable toolchain with `cargo build` only.
If you want to run tests, you will need the nightly toolchain, using 
`cargo test --no-default-features`.

[yaml-spec]: http://www.yaml.org/spec/1.2/spec.html
