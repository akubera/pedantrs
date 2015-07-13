# pedantrs
A Rust linter, purely for educational purposes. It contains the following lints:

* (WARN) Check that the number of arguments accepted to each function is not
  excessively large.
* (WARN) Check that public constants are documented.
* (WARN) Check that functions don't use overly deep nesting of expressions
* (WARN) Check that private functions, traits, impls, modules, enums and structs
  are documented.

See the modules in `src/lints/` for implementation details. For further
information see the [Rust documentation][docs].

[docs]: https://doc.rust-lang.org/book/compiler-plugins.html#lint-plugins
