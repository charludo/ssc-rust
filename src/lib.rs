include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
use grammar::{Node, Rule};
extern crate state;
mod base_rules;
mod comparators;
mod helpers;
mod interpreter;
mod operators;
mod prefixes;
// use std::fs;
// use std::io::{Error, Write};
use wasm_bindgen::prelude::*;

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

fn visit<'a>(node: &'a Node, input: &'a str) -> Vec<Vec<Vec<String>>> {
    // println!("RULE: {:?\n}, VALUE: >{}<", node.rule, node.as_str(input));
    match node.rule {
        Rule::source => {
            if node.children[0].rule != Rule::order {
                ORDER.set(9);
            }
            let mut truths: Vec<Vec<Vec<String>>> = Vec::new();
            for child in &node.children {
                truths.append(&mut visit(child, input));
            }
            truths
        }
        Rule::order => {
            let order: u8 = node.children[0].as_str(input).trim().parse().unwrap();
            ORDER.set(u8::pow(order, 2));
            println!("the order of the puzzle is: {}", order);
            vec![vec![vec!["True".to_owned()]]]
        }
        Rule::proposition => {
            let left = visit(&node.children[0], input);
            let comparator = node.children[1].as_str(input).trim();
            let right = visit(&node.children[2], input);

            let mut results: Vec<Vec<Vec<String>>> = Vec::new();
            for i in &left {
                for j in &right {
                    results.push(comparators::compare(i.to_vec(), comparator, j.to_vec()))
                }
            }
            results
        }
        Rule::builtin => {
            let prefix = node.children[0].as_str(input).trim();
            let mut args: Vec<Vec<Vec<String>>> = Vec::new();
            for arg in &node.children[1..] {
                args.append(&mut visit(arg, input));
            }

            return prefixes::builtin(prefix, args);
        }
        Rule::expression => {
            if node.children.len() == 1 {
                return visit(&node.children[0], input);
            }

            let left = visit(&node.children[0], input);
            let operator = node.children[1].as_str(input).trim();
            let right = visit(&node.children[2], input);

            let mut results: Vec<Vec<Vec<String>>> = Vec::new();
            for i in &left {
                for j in &right {
                    results.push(operators::operate(i.to_vec(), operator, j.to_vec()));
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
                atoms.push(vec![format!("{}_{}", cell.trim(), i)]);
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

#[wasm_bindgen]
pub fn compile(description: &str) -> String {
    let mut parser = grammar::PEG::new();

    // let args: Vec<String> = std::env::args().collect();

    // if args.len() != 2 {
        // eprintln!("Usage {} DESCRIPTION", &args[0]);
        // std::process::exit(2);
    // }

    // let input = &fs::read_to_string(&args[1])?.replace('\n', "");
    // let input = &args[1];
    let input = &description.replace('\n', "");

    match parser.parse(input) {
        Ok(node) => {
            let mut nodes: Vec<Node> = Vec::new();
            flatten_rec(node, &mut nodes);

            let propositions = visit(&nodes[0], input);
            let formula: String = helpers::reduce(propositions, "and");
            let rules: String = base_rules::get_base_rules();

            // let path = "test.sat";
            // let mut file = fs::File::create(path)?;

            // if let Err(e) = write!(file, "{} & True & !ERR & {}", formula, rules) {
            // eprintln!("Couldn't write to file: {}", e);
            // }

            // interpreter::solve(path);
            return format!("{} & True & !ERR & {}", formula, rules);
        }
        Err((line_no, col_no)) => {
            return format!("parser error at {}:{}", line_no, col_no);
        }
    }
}

#[wasm_bindgen]
pub fn interpret(result: &str) {
    interpreter::postprocess(result);
}

#[wasm_bindgen]
extern "C" {
    pub fn limboole(s: &str) -> String;
    pub fn display(s: String);
}
