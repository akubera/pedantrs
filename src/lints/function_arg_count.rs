//! Check that functions don't accept an "excessive" number of arguments.

use syntax::ast::{Block, FnDecl, NodeId};
use syntax::codemap::Span;
use syntax::visit::FnKind;
use rustc::lint::{EarlyContext, EarlyLintPass, LintArray, LintPass, LintContext};

const MAX_ARGS_DEFAULT: usize = 6;
const MAX_ARGS_FOR_CLOSURE: usize = 4;

declare_lint!(FN_ARG_LIST_LENGTH, Warn, "Warn about long argument lists");

pub struct Pass;

fn get_max_args_allowed(kind: &FnKind) -> usize {
    if let &FnKind::Closure = kind {
        MAX_ARGS_FOR_CLOSURE
    } else {
        MAX_ARGS_DEFAULT
    }
}

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(FN_ARG_LIST_LENGTH)
    }
}

impl EarlyLintPass for Pass {
    fn check_fn(&mut self,
                cx: &EarlyContext,
                kind: FnKind,
                decl: &FnDecl,
                _: &Block,
                span: Span,
                _: NodeId) {
        if decl.inputs.len() > get_max_args_allowed(&kind) {
            cx.span_lint(FN_ARG_LIST_LENGTH, span, 
                         "function has an excessive number of arguments");
        }
    }
}
