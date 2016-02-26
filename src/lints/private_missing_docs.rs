//! Check that private traits, impls, functions and const items have docs

use syntax::ast::{Item, ItemKind, Visibility};
use rustc::lint::{EarlyContext, EarlyLintPass, LintArray, LintPass, LintContext};

declare_lint!(PRIV_MISSING_DOCS, Warn,
    "Warn about private traits, impls, functions and const items without \
    documentation");

pub struct Pass;

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(PRIV_MISSING_DOCS)
    }
}

impl EarlyLintPass for Pass {
    fn check_item(&mut self, cx: &EarlyContext, i: &Item) {
        match i.node {
            ItemKind::Const(..) | ItemKind::Fn(..) |
                ItemKind::Impl(..) | ItemKind::Trait(..) |
                ItemKind::Struct(..) | ItemKind::Enum(..) |
                ItemKind::Mod(..) => {
                    if let Visibility::Public = i.vis {
                        // Publicly visible items are handled by other lints
                        return;
                    }

                    let doc_found = i.attrs.iter().find(
                        |a| a.node.is_sugared_doc);
                    if let None = doc_found {
                        cx.span_lint(PRIV_MISSING_DOCS, i.span,
                                    "private item is missing documentation");
                    }
            },

            _ => return,
        }
    }
}
