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
        Rule::Terminal | Rule::prop | Rule::exp => {
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

fn walk<'a>(node: &'a Node, input: &'a str) -> Vec<String> {
    // println!("RULE: {:?\n}, VALUE: >{}<", node.rule, node.as_str(input));
    match node.rule {
        Rule::source => {
            let mut truths: Vec<String> = Vec::new();
            for child in &node.children {
                // println!("here");
                truths.append(&mut walk(child, input));
            }
            truths
        }
        Rule::order => {
            let order: u8 = node.children[0].as_str(input).trim().parse().unwrap();
            println!("the order of the puzzle is: {}", order);
            vec![]
        }
        Rule::proposition => {
            let mut truths: Vec<String> = Vec::new();
            for child in &node.children {
                truths.append(&mut walk(child, input));
            }
            truths
        }
        Rule::expression => {
            vec![node.as_str(input).to_owned()]
        }
        Rule::builtin => {
            vec![node.as_str(input).to_owned()]
        }
        Rule::args => {
            vec![node.as_str(input).to_owned()]
        }
        Rule::value => {
            vec![node.as_str(input).to_owned()]
        }
        Rule::list => {
            vec![node.as_str(input).to_owned()]
        }
        Rule::CELL => {
            vec![node.as_str(input).to_owned()]
        }
        Rule::NUMBER => {
            let n: u8 = node.as_str(input).trim().parse().unwrap();
            vec![n.to_string()]
        }
        Rule::OPERATOR => {
            vec![node.as_str(input).to_owned()]
        }
        Rule::COMPARATOR => {
            vec![node.as_str(input).to_owned()]
        }
        Rule::PREFIX => {
            vec!["prefix".to_owned()]
        }
        Rule::Terminal => {
            vec![node.as_str(input).to_owned()]
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
            println!("result: {:?}", walk(&nodes[0], input));
        }
        Err((line_no, col_no)) => {
            eprintln!("parser error at {}:{}", line_no, col_no);
        }
    }
}
