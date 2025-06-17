use crate::node::{ComponentParameterValue, Node, RustBlockContent, SectionDirectiveContent, TextBlockItem, TextLineItem};

fn print_indent(indent: usize) {
    print!("{}", "  ".repeat(indent));
}

fn view_text_line_item(item: &TextLineItem, indent: usize) {
    print_indent(indent);
    match item {
        TextLineItem::Text(text, _) => println!("- Text: {:?}", text),
        TextLineItem::RustExprSimple(expr, _, _) => println!("- RustExprSimple: {:?}", expr),
    }
}

fn view_text_block_item(item: &TextBlockItem, indent: usize) {
    print_indent(indent);
    match item {
        TextBlockItem::Text(text, _) => println!("- Text: {:?}", text),
        TextBlockItem::RustExprSimple(expr, _, _) => println!("- RustExprSimple: {:?}", expr),
    }
}

fn view_rust_block_content(content: &RustBlockContent, indent: usize) {
    print_indent(indent);
    match content {
        RustBlockContent::Code(code, _) => {
            println!("- Code: {:?}", code);
        }
        RustBlockContent::TextLine(items, _) => {
            println!("- TextLine:");
            for item in items {
                view_text_line_item(item, indent + 1);
            }
        }
        RustBlockContent::TextBlock(items, _) => {
            println!("- TextBlock:");
            for item in items {
                view_text_block_item(item, indent + 1);
            }
        }
        RustBlockContent::NestedBlock(contents, _) => {
            println!("- NestedBlock:");
            for inner_content in contents {
                view_rust_block_content(inner_content, indent + 1);
            }
        }
    }
}

pub fn view_node(node: &Node, indent: usize) {
    print_indent(indent);
    match node {
        Node::Template(nodes, _) => {
            println!("- Template:");
            for inner_node in nodes {
                view_node(inner_node, indent + 1);
            }
        }
        Node::Text(text, _) => {
            println!("- Text: {:?}", text);
        }
        Node::InnerText(text, _) => {
            println!("- InnerText: {:?}", text);
        }
        Node::Comment(comment, _) => {
            println!("- Comment: {:?}", comment);
        }
        // Node::IncludeDirective(path) => {
        //     println!("- IncludeDirective: {:?}", path);
        // }
        Node::ExtendsDirective((path, _), _, _) => {
            println!("- ExtendsDirective: {:?}", path);
        }
        Node::RenderDirective(path, _) => {
            println!("- RenderDirective: {:?}", path);
        }
        Node::RustBlock(contents, _) => {
            println!("- RustBlock:");
            for content in contents {
                view_rust_block_content(content, indent + 1);
            }
        }
        Node::RustExprSimple(expr, _, _) => {
            println!("- RustExprSimple: {:?}", expr);
        }
        Node::RustExprParen(expr, _, _) => {
            println!("- RustExprParen: {:?}", expr);
        }
        Node::RustExpr(clauses, _) => {
            println!("- RustExpr:");
            for ((condition, _), nodes) in clauses {
                print_indent(indent + 1);
                println!("- Clause: {:?}", condition);
                for inner_node in nodes {
                    view_node(inner_node, indent + 2);
                }
            }
        }
        Node::MatchExpr((head, _), arms, _) => {
            println!("- MatchExpr:");
            print_indent(indent + 1);
            println!("- Clause: {:?}", head);
            print_indent(indent + 1);
            println!("- Arms:");
            for ((head, _), values) in arms {
                print_indent(indent + 2);
                println!("- Arm: {:?}", head);
                for inner_node in values {
                    view_node(inner_node, indent + 3);
                }
            }
        }
        Node::SectionDirective((name, _), (body, _), _) => {
            println!("- SectionDirective:");
            print_indent(indent + 1);
            println!("- StringLine: {:?}", name);
            print_indent(indent + 1);
            match body {
                SectionDirectiveContent::Text(s) => println!("- StringLine: {:?}", s),
                SectionDirectiveContent::RustExprSimple(s, _) => println!("- RustExprSimple: {:?}", s),
            }
        }
        Node::SectionBlock((section_head, _), body, _) => {
            println!("- SectionBlock:");
            print_indent(indent + 1);
            println!("- StringLine: {:?}", section_head);
            for inner_node in body {
                view_node(inner_node, indent + 1);
            }
        }
        Node::RenderBody(_) => {
            println!("- RenderBody");
        }
        Node::Component((name, _), parameters, body, _) => {
            println!("- Component:");
            print_indent(indent + 1);
            println!("- Name: {:?}", name);
            print_indent(indent + 1);
            println!("- Parameters:");
            for (parameter, _) in parameters {
                print_indent(indent + 2);
                println!("- Name: {:?}", parameter.name);
                print_indent(indent + 2);
                match &parameter.value {
                    ComponentParameterValue::Bool(b) => println!("- Bool: {:?}", b),
                    ComponentParameterValue::Number(b) => println!("- Number: {:?}", b),
                    ComponentParameterValue::String(s) => println!("- String: {:?}", s),
                    ComponentParameterValue::RustExprSimple(s) => {
                        println!("- RustExprSimple: {:?}", s)
                    }
                    ComponentParameterValue::RustExprParen(s) => {
                        println!("- RustExprParen: {:?}", s)
                    }
                    ComponentParameterValue::Block(nodes) => {
                        println!("- Block:");
                        for node in nodes {
                            view_node(node, indent + 3)
                        }
                    }
                }
            }
            for inner_node in body {
                view_node(inner_node, indent + 1);
            }
        }
        Node::ChildContent(_) => {
            println!("- ChildContent");
        }
        Node::Raw(s, _) => println!("- Raw: {:?}", s),
        Node::UseDirective(component_name, (import_path, _), component, _) => {
            println!("- UseDirective:");
            print_indent(indent + 1);
            println!("- ComponentName: {:?}", component_name);
            print_indent(indent + 1);
            println!("- ImportPath: {:#?}", import_path);
            print_indent(indent + 1);
            println!("- Component:");
            view_node(component, indent + 2);
        }
        Node::ContinueDirective(_) => {
            println!("- ContinueDirective");
        }
        Node::BreakDirective(_) => {
            println!("- BreakDirective");
        }
    }
}
