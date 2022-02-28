include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
use grammar::{Node, Rule};
extern crate state;

static ORDER: state::Storage<u8> = state::Storage::new();

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
        Rule::Terminal | Rule::value => {
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

fn operator_placeholder1(left: &Vec<String>, right: &Vec<String>) -> Vec<String> {
    vec!["some operator 1".to_owned()]
}

fn operator_placeholder2(left: &Vec<String>, right: &Vec<String>) -> Vec<String> {
    vec!["some operator 2".to_owned()]
}

fn visit<'a>(node: &'a Node, input: &'a str) -> Vec<String> {
    // println!("RULE: {:?\n}, VALUE: >{}<", node.rule, node.as_str(input));
    match node.rule {
        Rule::source => {
            let mut truths: Vec<String> = Vec::new();
            for child in &node.children {
                truths.append(&mut visit(child, input));
            }
            truths
        }
        Rule::order => {
            let order: u8 = node.children[0].as_str(input).trim().parse().unwrap();
            ORDER.set(order);
            println!("the order of the puzzle is: {}", order);
            vec![]
        }
        Rule::proposition => {
            let left = visit(&node.children[0], input);
            let comparator = node.children[1].as_str(input).trim();
            let right = visit(&node.children[2], input);

            println!(">{:?}<", comparator);

            match comparator {
                ">" => vec!["greater than".to_owned()],
                _ => vec!["something else!".to_owned()],
            }
        }
        Rule::builtin => {
            let prefix = node.children[0].as_str(input).trim();
            let mut args: Vec<String> = Vec::new();
            for arg in &node.children[1..] {
                args.append(&mut visit(arg, input));
            }

            println!(">{:?}<", prefix);

            match prefix {
                "!!" => vec!["distinct".to_owned()],
                _ => vec!["something else!".to_owned()],
            }
        }
        Rule::expression => {
            if node.children.len() == 1 {
                return visit(&node.children[0], input);
            }

            let left = visit(&node.children[0], input);
            let operator = node.children[1].as_str(input).trim();
            let right = visit(&node.children[2], input);

            let op_fn: fn(left: &Vec<String>, right: &Vec<String>) -> Vec<String>;
            op_fn = match operator {
                "+" => operator_placeholder1,
                _ => operator_placeholder2,
            };

            let mut results: Vec<String> = Vec::new();
            for i in left {
                for j in right {
                    results.append(op_fn(i, j))
                }
            }
            results
        }
        Rule::list => {
            let mut values: Vec<String> = Vec::new();
            for child in &node.children {
                values.append(&mut visit(child, input));
            }
            values
        }
        Rule::CELL => {
            let cell: &str = node.as_str(input).trim();
            let mut atoms: Vec<String> = Vec::new();
            for i in 1..=*ORDER.get() {
                atoms.push(format!("{}_{}", cell, i));
            }
            atoms
        }
        Rule::NUMBER => {
            let n: u8 = node.as_str(input).trim().parse().unwrap();
            let mut atoms: Vec<String> = Vec::new();
            for i in 1..*ORDER.get() {
                atoms.push("False".to_owned());
            }
            atoms.push("True".to_owned());
            atoms
        }
        _ => {
            println!("{:?\n}", node);
            unreachable!()
        }
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
            let mut nodes: Vec<Node> = Vec::new();
            flatten_rec(node, &mut nodes);
            // println!("{:?}", nodes);
            println!("result: {:?}", visit(&nodes[0], input));
        }
        Err((line_no, col_no)) => {
            eprintln!("parser error at {}:{}", line_no, col_no);
        }
    }
}
