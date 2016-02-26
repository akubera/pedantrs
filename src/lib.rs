#![deny(missing_docs)]

#![feature(plugin_registrar)]
#![feature(box_syntax, rustc_private)]

#![feature(plugin)]
#![plugin(clippy)]

//! A simple linter, written for educational purposes. The following lints are
//! provided:
//!
//! * (WARN) Check that the number of arguments accepted to each function is
//!     not excessively large
//! * (WARN) Check that functions nest expressions excessively deeply
//! * (WARN) Check that public and private constants are documented
//! * (WARN) Check that private functions, structs, enums, traits, impls,
//!     mods and constants are documented. Their public equivalents are checked
//!     by the compiler-provided 'missing_docs' lint.

extern crate syntax;
// Load rustc as a plugin to get macros
#[macro_use]
extern crate rustc;
extern crate rustc_plugin;

use rustc::lint::EarlyLintPassObject;
use rustc_plugin::Registry;

mod lints;

/// Register the lints
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_early_lint_pass(
        box lints::function_arg_count::Pass as EarlyLintPassObject);
    reg.register_early_lint_pass(
        box lints::const_missing_docs::Pass as EarlyLintPassObject);
    reg.register_early_lint_pass(
        box lints::expression_nesting::Pass as EarlyLintPassObject);
    reg.register_early_lint_pass(
        box lints::private_missing_docs::Pass as EarlyLintPassObject);
}
