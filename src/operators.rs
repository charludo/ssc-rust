use crate::helpers;
use itertools::iproduct;
use std::collections::HashSet;

pub fn operate(
    left: Vec<Vec<String>>,
    operator: &str,
    right: Vec<Vec<String>>,
) -> Vec<Vec<String>> {
    if operator == "|" {
        return or(left, right);
    }

    let op_fn: fn(left: usize, right: usize) -> (Vec<Vec<Vec<usize>>>, usize);
    op_fn = match operator {
        "+" => add,
        "-" => sub,
        "*" => mult,
        _ => unreachable!(),
    };

    let (value_map, max_len) = op_fn(left.len(), right.len());
    let (left, right) = helpers::equalize(left, right);
    let mut values: Vec<Vec<String>> = Vec::new();

    for i in 0..max_len {
        let mut vals: Vec<String> = Vec::new();
        for pair in &value_map[i] {
            let l: usize = pair[0];
            let r: usize = pair[1];
            let mut options: Vec<String> = helpers::and_clause(left[l].to_vec(), right[r].to_vec());
            vals.append(&mut options);
        }
        vals = vals
            .iter()
            .map(|v| v.to_string())
            .filter(|v| *v != "False".to_string())
            .collect();
        if vals.len() < 1 {
            vals = vec!["False".to_owned()];
        }
        values.push(vals);
    }
    values
}

fn add(left: usize, right: usize) -> (Vec<Vec<Vec<usize>>>, usize) {
    let mut legal_pairs: Vec<Vec<Vec<usize>>> = Vec::new();
    for i in 0..left + right {
        let options: Vec<Vec<usize>> = iproduct!(0..left, 0..right)
            .filter(|(a, b)| a + b + 1 == i)
            .map(|(a, b)| vec![a, b])
            .collect();
        legal_pairs.push(options);
    }
    (legal_pairs, left + right)
}

fn sub(left: usize, _right: usize) -> (Vec<Vec<Vec<usize>>>, usize) {
    let mut legal_pairs: Vec<Vec<Vec<usize>>> = Vec::new();
    for i in 0..left {
        let options: Vec<Vec<usize>> = iproduct!(0..left, 0..left)
            .filter(|(a, b)| *a as i8 - *b as i8 - 1 == i as i8)
            .map(|(a, b)| vec![a, b])
            .collect();
        legal_pairs.push(options);
    }
    (legal_pairs, left)
}

fn mult(left: usize, right: usize) -> (Vec<Vec<Vec<usize>>>, usize) {
    let mut legal_pairs: Vec<Vec<Vec<usize>>> = Vec::new();
    for i in 0..left * right {
        let options: Vec<Vec<usize>> = iproduct!(0..left, 0..right)
            .filter(|(a, b)| (a + 1) * (b + 1) - 1 == i)
            .map(|(a, b)| vec![a, b])
            .collect();
        legal_pairs.push(options);
    }
    (legal_pairs, left * right)
}

fn or(left: Vec<Vec<String>>, right: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let (left, right) = helpers::equalize(left, right);
    let mut buffer: Vec<Vec<String>> = Vec::new();
    for i in 0..left.len() {
        let mut combined: Vec<String> = Vec::new();
        combined.append(&mut left[i].to_vec());
        combined.append(&mut right[i].to_vec());

        let mut set: HashSet<_> = combined.to_vec().drain(..).collect();
        set.remove("False");
        combined.extend(set.into_iter());
        println!("combined: {:?}", combined);

        buffer.push(combined)
    }
    println!("buffer: {:?}", buffer);
    buffer
}
