include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

use grammar::{Node, Rule};

fn main() {
    let mut parser = grammar::PEG::new();

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage {} DESCRIPTION", &args[0]);
        std::process::exit(2);
    }

    let input = &args[1];
    println!("parsing: {}", input);

    match parser.parse(input) {
        Ok(node) => {
            // fn drop_whitespace(children: Vec<Node>) -> Vec<Node> {
            //     children.into_iter().filter(|c| c.rule.to_string() != "WHITESPACE").collect::<Vec<Node>>()
            // }
            fn walk<'a>(node: &'a Node, input: &'a str) -> Vec<&'a str> {
                println!("RULE: {:?\n}, VALUE: >{}<", node.rule, node.as_str(input));
                match node.rule {
                    Rule::source => {
                        let mut truths: Vec<&str> = Vec::new();
                        for child in &node.children {
                            truths.append(&mut walk(child, input));
                        }
                        truths
                    }
                    Rule::order => {
                        let mut truths: Vec<&str> = Vec::new();
                        for child in &node.children {
                            truths.append(&mut walk(child, input));
                        }
                        truths
                    }
                    Rule::COMMENT => {
                        vec!["COMMENT"]
                    }
                    Rule::WS => {
                        vec!["WS"]
                    }
                    Rule::proposition => {
                        vec!["proposition"]
                    }
                    Rule::prop => {
                        vec!["prop"]
                    }
                    Rule::expression => {
                        vec!["expression"]
                    }
                    Rule::exp => {
                        vec!["exp"]
                    }
                    Rule::builtin => {
                        vec!["builtin"]
                    }
                    Rule::args => {
                        vec!["args"]
                    }
                    Rule::value => {
                        vec!["value"]
                    }
                    Rule::list => {
                        vec!["list"]
                    }
                    Rule::CELL => {
                        vec!["cell"]
                    }
                    Rule::NUMBER => {
                        let mut truths: Vec<&str> = Vec::new();
                        for child in &node.children {
                            truths.append(&mut walk(child, input));
                        }
                        truths
                    }
                    Rule::OPERATOR => {
                        vec!["operator"]
                    }
                    Rule::COMPARATOR => {
                        vec!["comparator"]
                    }
                    Rule::PREFIX => {
                        vec!["prefix"]
                    }
                    Rule::Terminal => {
                        vec![node.as_str(input)]
                    }
                    _ => {
                        println!("{:?\n}", node);
                        unreachable!()
                    }
                }
            }

            println!("result: {:?}", walk(&node, input));
        }
        Err((line_no, col_no)) => {
            eprintln!("parser error at {}:{}", line_no, col_no);
        }
    }
}
