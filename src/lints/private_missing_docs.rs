//! Check that public constants have documentation

use syntax::ast::{Item, Item_, Visibility};
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
            Item_::ItemConst(..) | Item_::ItemFn(..) | 
                Item_::ItemImpl(..) | Item_::ItemTrait(..) |
                Item_::ItemStruct(..) | Item_::ItemEnum(..) | 
                Item_::ItemMod(..) => {
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
