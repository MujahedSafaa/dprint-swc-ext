use swc_atoms::JsWord;
use swc_common::SyntaxContext;

use crate::BindingIdent;
use crate::Expr;
use crate::Ident;
use crate::OptChainBase;
use crate::OptChainExpr;

/// Redeclaration of `swc_ecma_utils::Id`.
/// Contains the name and scope of the identifier, but only
/// when the tree has been resolved with an swc resolver
/// such as ts_resolver.
pub type Id = (JsWord, SyntaxContext);

impl<'a> Ident<'a> {
  pub fn to_id(&self) -> Id {
    (self.sym().clone(), self.ctxt())
  }

  pub fn ctxt(&self) -> SyntaxContext {
    self.inner.span.ctxt
  }
}

impl<'a> BindingIdent<'a> {
  pub fn to_id(&self) -> Id {
    self.id.to_id()
  }

  pub fn ctxt(&self) -> SyntaxContext {
    self.id.ctxt()
  }
}

impl<'a> OptChainExpr<'a> {
  pub fn expr(&self) -> Expr {
    match self.base {
      OptChainBase::Member(member_expr) => member_expr.obj,
      OptChainBase::Call(call_expr) => call_expr.callee,
    }
  }
}
