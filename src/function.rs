use syntax::ast::*;
use syntax::codemap::{Span};
use syntax::ext::base::Annotatable;

#[derive(Debug)]
pub struct Function {
    ident: Ident,
    decl: FnDecl,
    attrs: Vec<Attribute>,
}

impl Function {
    pub fn from(annotated: &Annotatable) -> Result<Function, Span> {
        let function = match *annotated {
            Annotatable::Item(ref item) => match item.node {
                ItemKind::Fn(ref decl, ..) => {
                    Function {
                        ident: item.ident,
                        decl: decl.clone().into_inner(),
                        attrs: item.attrs.clone(),
                    }
                }
                _ => return Err(item.span)
            },
            Annotatable::TraitItem(ref item) => return Err(item.span),
            Annotatable::ImplItem(ref item) => return Err(item.span),
        };

        Ok(function)
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn attrs(&self) -> &Vec<Attribute> {
        &self.attrs
    }
}
