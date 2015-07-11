//! Check that functions don't contain excessive nesting of expressions

use syntax::ast::{Block, FnDecl, NodeId, Expr_, Expr, Stmt_};
use syntax::codemap::Span;
use syntax::visit::FnKind;
use rustc::lint::{Context, LintArray, LintPass};

const MAX_NESTING_DEPTH: u32 = 2;

declare_lint!(FN_EXPR_NESTING_DEPTH, Warn, 
    "Warn about deeply nested expressions");

pub struct Pass;

fn expr_to_blocks(e: &Expr) -> Vec<&Block> {
    match e.node {
        Expr_::ExprBlock(ref inner_block) => vec![&inner_block],

        Expr_::ExprIf(_, ref if_block, _) | 
            Expr_::ExprIfLet(_, _, ref if_block, _) => vec![&if_block],

        Expr_::ExprWhile(_, ref body, _) |
            Expr_::ExprWhileLet(_, _, ref body, _)  |
            Expr_::ExprLoop(ref body, _) => vec![&body],

        _ => return vec![],
    }
}

fn check_nesting(cx: &Context, b: &Block, level: u32) {
    if level > MAX_NESTING_DEPTH {
        cx.span_lint(FN_EXPR_NESTING_DEPTH, b.span, 
                     "function has excessive nesting of expressions");
        return;
    }

    // Blocks consist of a vector of statements ...
    for s in &b.stmts {
        match &s.node {
            &Stmt_::StmtExpr(ref e, _) | &Stmt_::StmtSemi(ref e, _) => {
                for inner_block in expr_to_blocks(&e) {
                    check_nesting(cx, inner_block, level + 1);
                }
                continue;
            }
            &Stmt_::StmtDecl(_, _) | &Stmt_::StmtMac(_, _) => continue,
        }
    } 

    // ... followed by an optional expression
    if let Some(ref e) = b.expr {
        for inner_block in expr_to_blocks(&e) {
            check_nesting(cx, inner_block, level + 1);
        }
    }
}

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(FN_EXPR_NESTING_DEPTH)
    }

    fn check_fn(&mut self, cx: &Context, _: FnKind, _: &FnDecl, block: &Block, 
                _: Span, _: NodeId) {
        check_nesting(cx, &block, 0);
    }
}
