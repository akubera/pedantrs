# pedantrs
A Rust linter, purely for educational purposes. It contains the following lints:

* (WARN) Check that the number of arguments accepted to each function is not
  excessively large.
* (WARN) Check that public constants are documented.
* (WARN) Check that functions don't use overly deep nesting of expressions.
* (WARN) Check that private functions, traits, impls, modules, enums and structs
  are documented.

See the modules in `src/lints/` for implementation details. For further
information see the [Rust documentation][docs].

## Building

Ensure you are using the [nightly build of Rust][nightly] (required for access
to the lint infrastructure), then simply run `cargo build` in the root
directory.

## Using

Adding `pedantrs`, or any compiler plugin, to the build process of your project
is a two step process. First, you need to update the project's `Cargo.toml` file
to add `pedantrs` as a depedency, then you need to enable the plugin at the
crate level within your project.

See the `demo` folder for an example of an application which makes use of the
linter. When you build this application `pedantrs` will be invoked and a series
of warnings generated.

[docs]: https://doc.rust-lang.org/book/compiler-plugins.html#lint-plugins
[nightly]: http://doc.rust-lang.org/book/nightly-rust.html
