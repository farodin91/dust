#![deny(missing_debug_implementations)]

#![feature(quote, concat_idents, plugin_registrar, rustc_private)]
#![feature(custom_attribute)]

extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::ext::base::{SyntaxExtension, MultiItemDecorator, ExtCtxt};
use syntax::ext::quote::rt::Span;
use syntax::ast::{self};
use syntax::ext::base::Annotatable;
use syntax::symbol::Symbol;

mod function;
mod ident_ext;

use function::Function;
use ident_ext::IdentExt;

struct Theory;

impl MultiItemDecorator for Theory {
    fn expand(
        &self,
        ecx: &mut ExtCtxt,
        sp: Span,
        _: &ast::MetaItem,
        item: &Annotatable,
        push: &mut FnMut(Annotatable)
    ) {
        let function = Function::from(item).unwrap_or_else(|item_sp| {
            ecx.span_err(sp, "this attribute can only be used on functions...");
            ecx.span_fatal(item_sp, "...but was applied to the item above.");
        });

        let original_fn_name = function.ident();

        for (data_fn_name, tokens) in function.attrs()
            .into_iter()
            .filter(|s| s.check_name("data"))
            .map(|v| v.clone().tokens.as_tree())
            .filter(|&(_, k)| !k)
            .map(|(s, _)| s)
            .enumerate()
            .map(|(i, a)| (original_fn_name.append(format!("_{}", i)),a)) {
            push(Annotatable::Item(quote_item!(ecx,
        #[test]
        fn $data_fn_name() {
                $original_fn_name$tokens;
            }

        ).expect("function")));
        }
    }
}

struct Data;/*  */

impl MultiItemDecorator for Data {
    fn expand(
        &self,
        ecx: &mut ExtCtxt,
        sp: Span,
        _: &ast::MetaItem,
        item: &Annotatable,
        _: &mut FnMut(Annotatable)
    ) {
        Function::from(item).unwrap_or_else(|item_sp| {
            ecx.span_err(sp, "this attribute can only be used on functions...");
            ecx.span_fatal(item_sp, "...but was applied to the item above.");
        });
    }
}


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let theory = Theory;
    reg.register_syntax_extension(Symbol::intern("theory"), SyntaxExtension::MultiDecorator(Box::new(theory)));
    let data = Data;
    reg.register_syntax_extension(Symbol::intern("data"), SyntaxExtension::MultiDecorator(Box::new(data)));
}
