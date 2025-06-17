use crate::Node;
use crate::compiler::Compiler;
use crate::node::{RustBlockContent, TextBlockItem, TextLineItem};
use anyhow::{Result, anyhow};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

pub struct RustBlockCompiler;

impl RustBlockCompiler {
    pub fn compile(compiler: &mut Compiler, contents: &Vec<RustBlockContent>) -> Result<TokenStream> {
        let mut token_stream = TokenStream::new();

        for content in contents {
            match content {
                RustBlockContent::Code(code, _) => {
                    let code_ts = TokenStream::from_str(code).map_err(|err| anyhow!("Lex Error: {}", err))?;
                    token_stream.extend(quote! { #code_ts });
                }
                RustBlockContent::TextLine(items, _) => {
                    for item in items {
                        match item {
                            TextLineItem::Text(text, position) => {
                                let t_ts = compiler.compile(&Node::Text(text.clone(), position.to_owned()))?;
                                token_stream.extend(quote! {#t_ts});
                            }
                            TextLineItem::RustExprSimple(expr, is_escaped, position) => {
                                let rxs_ts = compiler.compile(&Node::RustExprSimple(expr.clone(), *is_escaped, position.to_owned()))?;
                                token_stream.extend(quote! {#rxs_ts});
                            }
                        }
                    }
                }
                RustBlockContent::TextBlock(items, _) => {
                    for item in items {
                        match item {
                            TextBlockItem::Text(text, position) => {
                                let t_ts = compiler.compile(&Node::Text(text.clone(), position.to_owned()))?;
                                token_stream.extend(quote! {#t_ts});
                            }
                            TextBlockItem::RustExprSimple(expr, is_escaped, position) => {
                                let rxs_ts = compiler.compile(&Node::RustExprSimple(expr.clone(), *is_escaped, position.to_owned()))?;
                                token_stream.extend(quote! {#rxs_ts});
                            }
                        }
                    }
                }
                RustBlockContent::NestedBlock(nested_contents, _) => {
                    let nested_ts = Self::compile(compiler, nested_contents)?;
                    token_stream.extend(quote! { {#nested_ts} });
                }
            }
        }

        Ok(token_stream)
    }
}
