include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
use grammar::{Node, Rule};
extern crate state;
mod comparators;
mod helpers;
mod operators;
mod prefixes;

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

fn operator_placeholder1(left: &Vec<Vec<String>>, right: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    vec![vec!["some operator 1".to_owned()]]
}

fn operator_placeholder2(left: &Vec<Vec<String>>, right: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    vec![vec!["some operator 2".to_owned()]]
}

fn visit<'a>(node: &'a Node, input: &'a str) -> Vec<Vec<Vec<String>>> {
    // println!("RULE: {:?\n}, VALUE: >{}<", node.rule, node.as_str(input));
    match node.rule {
        Rule::source => {
            let mut truths: Vec<Vec<Vec<String>>> = Vec::new();
            for child in &node.children {
                truths.append(&mut visit(child, input));
            }
            truths
        }
        Rule::order => {
            let order: u8 = node.children[0].as_str(input).trim().parse().unwrap();
            ORDER.set(order ^ 2);
            println!("the order of the puzzle is: {}", order);
            vec![vec![vec![]]]
        }
        Rule::proposition => {
            let left = visit(&node.children[0], input);
            let comparator = node.children[1].as_str(input).trim();
            let right = visit(&node.children[2], input);

            println!(">{:?}<", comparator);

            match comparator {
                ">" => vec![vec![vec!["greater than".to_owned()]]],
                _ => vec![vec![vec!["something else!".to_owned()]]],
            }
        }
        Rule::builtin => {
            let prefix = node.children[0].as_str(input).trim();
            let mut args: Vec<Vec<Vec<String>>> = Vec::new();
            for arg in &node.children[1..] {
                args.append(&mut visit(arg, input));
            }

            println!(">{:?}<", prefix);

            match prefix {
                "!!" => vec![vec![vec!["distinct".to_owned()]]],
                _ => vec![vec![vec!["something else!".to_owned()]]],
            }
        }
        Rule::expression => {
            if node.children.len() == 1 {
                return visit(&node.children[0], input);
            }

            let left = visit(&node.children[0], input);
            let operator = node.children[1].as_str(input).trim();
            let right = visit(&node.children[2], input);

            let op_fn: fn(left: &Vec<Vec<String>>, right: &Vec<Vec<String>>) -> Vec<Vec<String>>;
            op_fn = match operator {
                "+" => operator_placeholder1,
                _ => operator_placeholder2,
            };

            let mut results: Vec<Vec<Vec<String>>> = Vec::new();
            for i in &left {
                for j in &right {
                    results.push(op_fn(&i, &j))
                }
            }
            results
        }
        Rule::list => {
            let mut values: Vec<Vec<Vec<String>>> = Vec::new();
            for child in &node.children {
                values.append(&mut visit(child, input));
            }
            values
        }
        Rule::CELL => {
            let cell: &str = node.as_str(input).trim();
            let mut atoms: Vec<Vec<String>> = Vec::new();
            for i in 1..=*ORDER.get() {
                atoms.push(vec![format!("{}_{}", cell, i)]);
            }
            vec![atoms]
        }
        Rule::NUMBER => {
            let n: u8 = node.as_str(input).trim().parse().unwrap();
            let mut atoms: Vec<Vec<String>> = Vec::new();
            for _ in 1..n {
                atoms.push(vec!["False".to_owned()]);
            }
            atoms.push(vec!["True".to_owned()]);
            vec![atoms]
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

    ORDER.set(3);
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
