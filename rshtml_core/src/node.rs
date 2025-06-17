use crate::parser::Rule;
use pest::iterators::Pair;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum TextBlockItem {
    Text(String, Position),
    RustExprSimple(String, bool, Position),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextLineItem {
    Text(String, Position),
    RustExprSimple(String, bool, Position),
}

#[derive(Debug, PartialEq, Clone)]
pub enum RustBlockContent {
    Code(String, Position),
    TextLine(Vec<TextLineItem>, Position),
    TextBlock(Vec<TextBlockItem>, Position),
    NestedBlock(Vec<RustBlockContent>, Position),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SectionDirectiveContent {
    Text(String),
    RustExprSimple(String, bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ComponentParameterValue {
    Bool(bool),
    Number(String),
    String(String),
    RustExprParen(String),
    RustExprSimple(String),
    Block(Vec<Node>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComponentParameter {
    pub name: String,
    pub value: ComponentParameterValue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    //IncludeDirective(PathBuf),         // include directive @include("other_view.html")
    Template(Vec<Node>, Position),                                                           // main template, contains child nodes
    Text(String, Position),                                                                  // plain text content (@@ -> @)
    InnerText(String, Position),                                                             // text inside a block (@@ -> @, @{ -> {, @} -> })
    Comment(String, Position),                                                               // comment content
    ExtendsDirective((PathBuf, Position), Box<Node>, Position),                              // extends directive @extends("layout.html")
    RenderDirective(String, Position),                                                       // yield directive @yield("content")
    RustBlock(Vec<RustBlockContent>, Position),                                              // @{ ... } block content (with trim)
    RustExprSimple(String, bool, Position),                                                  // @expr ... (simple expression)
    RustExprParen(String, bool, Position),                                                   // @(expr) (expression parentheses)
    MatchExpr((String, Position), Vec<((String, Position), Vec<Node>)>, Position),           // @match expr { ... => ... }
    RustExpr(Vec<((String, Position), Vec<Node>)>, Position),                                // @if ...  { ... } else { ... } / @for ... { ... }
    SectionDirective((String, Position), (SectionDirectiveContent, Position), Position),     // @section("content")
    SectionBlock((String, Position), Vec<Node>, Position),                                   // @section content { ... }
    RenderBody(Position),                                                                    // @render_body (main body of subpage)
    Component((String, Position), Vec<(ComponentParameter, Position)>, Vec<Node>, Position), // @componentName(param1 = value1, param2 = value2) { ... } also <CompName p=""/> tags
    ChildContent(Position),                                                                  // @child_content (component child content)
    Raw(String, Position),                                                                   // @raw {} (raw content)
    UseDirective(String, (PathBuf, Position), Box<Node>, Position),                          // @use "component.rs.html" as Component
    ContinueDirective(Position),                                                             // @continue for the loops
    BreakDirective(Position),                                                                // @break for the loops
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Position((usize, usize), (usize, usize)); // start: (line, col), end: (line, col)

impl From<&Pair<'_, Rule>> for Position {
    fn from(value: &Pair<Rule>) -> Self {
        Self(value.as_span().start_pos().line_col(), value.as_span().end_pos().line_col())
    }
}
