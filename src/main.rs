include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

use grammar::{Node, Rule};

fn flatten(node: Node) -> Node {
    let mut buffer = vec![];
    for node in node.children {
        flatten_rec(node, &mut buffer)
    }
    Node {
        rule: node.rule,
        start: node.start,
        end: node.end,
        children: buffer,
        alternative: node.alternative,
    }
}

fn flatten_rec(node: Node, buffer: &mut Vec<Node>) {
    match node.rule {
        // flatten these nodes
        Rule::Terminal if node.start == node.end => {}
        Rule::Terminal => {
            for node in node.children {
                flatten_rec(node, buffer)
            }
        }
        // not important
        Rule::EOI => {}
        // #[cfg(feature = "no-ignored")]
        Rule::IGNORE => {}
        // #[cfg(not(feature = "no-ignored"))]
        // Rule::IGNORE if node.start == node.end => {}
        // #[cfg(feature = "no-unnamed")]
        Rule::WHITESPACE => {}
        // #[cfg(not(feature = "no-unnamed"))]
        _ => buffer.push(flatten(node)),
    }
}

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
            fn walk<'a>(node: &'a Node, input: &'a str) -> Vec<&'a str> {
                println!("RULE: {:?\n}, VALUE: >{}<", node.rule, node.as_str(input));
                match node.rule {
                    Rule::source => {
                        let mut truths: Vec<&str> = Vec::new();
                        for child in &node.children {
                            // println!("here");
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
                    Rule::EOI => {
                        vec![]
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
                        let mut truths: Vec<&str> = Vec::new();
                        for child in &node.children {
                            truths.append(&mut walk(child, input));
                        }
                        truths
                    }
                    _ => {
                        println!("{:?\n}", node);
                        unreachable!()
                    }
                }
            }
            let mut nodes: Vec<Node> = Vec::new();
            flatten_rec(node, &mut nodes);
            // println!("{:?}", nodes);
            println!("result: {:?}", walk(&nodes[0], input));
        }
        Err((line_no, col_no)) => {
            eprintln!("parser error at {}:{}", line_no, col_no);
        }
    }
}
