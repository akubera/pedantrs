//! Check that public constants have documentation

use syntax::ast::{Item, ItemKind, Visibility};
use rustc::lint::{EarlyContext, EarlyLintPass, LintArray, LintPass, LintContext};

declare_lint!(PUB_CONST_DOCS, Warn,
    "Warn about public const items without documentation");

pub struct Pass;

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(PUB_CONST_DOCS)
    }
}

impl EarlyLintPass for Pass {
    fn check_item(&mut self, cx: &EarlyContext, i: &Item) {
        if let (&ItemKind::Const(..), Visibility::Public) = (&i.node, i.vis) {
            let doc_found = i.attrs.iter().find(|a| a.node.is_sugared_doc);
            if let None = doc_found {
                cx.span_lint(PUB_CONST_DOCS, i.span,
                            "public constant is missing documentation");
            }
        }
    }
}
