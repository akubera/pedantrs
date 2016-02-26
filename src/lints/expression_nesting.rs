//! Check that functions don't contain excessive nesting of expressions

extern crate rustc_front;

use syntax::ast::{Block, FnDecl, NodeId, Expr, ExprKind, StmtKind};
use syntax::codemap::Span;
use syntax::visit::FnKind;
use self::rustc_front::hir::{Expr_, Stmt_};
use rustc::lint::{EarlyContext, EarlyLintPass, LintArray, LintPass, LintContext};

const MAX_NESTING_DEPTH: u32 = 2;

declare_lint!(FN_EXPR_NESTING_DEPTH, Warn,
    "Warn about deeply nested expressions");

pub struct Pass;

fn expr_to_blocks(e: &Expr) -> Vec<&Block> {
    match e.node {
        ExprKind::Block(ref inner_block) => vec![&inner_block],

        ExprKind::If(_, ref if_block, ref else_opt) |
            ExprKind::IfLet(_, _, ref if_block, ref else_opt) =>
            {
                if let Some(ref else_expr) = *else_opt {
                    let mut else_blocks = expr_to_blocks(&else_expr);
                    else_blocks.push(&if_block);
                    else_blocks
                } else {
                    vec![&if_block]
                }
            },

        ExprKind::While(_, ref body, _) |
            ExprKind::WhileLet(_, _, ref body, _)  |
            ExprKind::Loop(ref body, _) => vec![&body],

        ExprKind::Match(_, ref arms) => {
            // For match expressions we ignore the nesting introduced by the
            // 'match' and just consider the arms
            let mut blocks = Vec::new();
            for ref a in arms {
                let mut arm_blocks = expr_to_blocks(&a.body);
                blocks.append(&mut arm_blocks);
            }
            blocks
        },

        _ => return vec![],
    }
}

fn check_nesting(cx: &EarlyContext, b: &Block, level: u32) {
    if level > MAX_NESTING_DEPTH {
        cx.span_lint(FN_EXPR_NESTING_DEPTH, b.span,
                     "function has excessive nesting of expressions");
        return;
    }

    // Blocks consist of a vector of statements ...
    for s in &b.stmts {
        match s.node {
            StmtKind::Expr(ref e, _) | StmtKind::Semi(ref e, _) => {
                for inner_block in expr_to_blocks(&e) {
                    check_nesting(cx, inner_block, level + 1);
                }
                continue;
            }
            StmtKind::Decl(_, _) | StmtKind::Mac(_, _, _) => continue,
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
}

impl EarlyLintPass for Pass {
    fn check_fn(&mut self,
                cx: &EarlyContext,
                _: FnKind,
                _: &FnDecl,
                block: &Block,
                _: Span,
                _: NodeId) {
        check_nesting(cx, &block, 0);
    }
}
