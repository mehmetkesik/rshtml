use crate::Node;
use crate::compiler::Compiler;
use crate::node::{Position, SectionDirectiveContent};
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

pub struct SectionDirectiveCompiler;

impl SectionDirectiveCompiler {
    pub fn compile(compiler: &mut Compiler, name: &str, content: &SectionDirectiveContent, position: &Position) -> Result<TokenStream> {
        let content_ts = match content {
            SectionDirectiveContent::Text(text) => compiler.compile(&Node::Text(text.clone(), position.to_owned()))?,
            SectionDirectiveContent::RustExprSimple(expr, is_escaped) => {
                compiler.compile(&Node::RustExprSimple(expr.clone(), *is_escaped, position.to_owned()))?
            }
        };

        compiler.sections.insert(name.to_owned(), content_ts.clone());

        Ok(quote! {})
    }
}
