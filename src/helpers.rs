use crate::ORDER;
use itertools::iproduct;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

fn is_atomic(clause: &String) -> bool {
    if clause.contains("&") || clause.contains("|") {
        false
    } else {
        true
    }
}

fn make_atomic(clause: String) -> Vec<String> {
    let re = Regex::new(r"[()\s]").unwrap();
    let cleansed = re.replace_all(&clause, "");

    let re = Regex::new(r"&|\|").unwrap();
    re.split(&cleansed)
        .into_iter()
        .map(|s| s.to_owned())
        .collect()
}

fn deconstruct(atom: &String) -> (String, String, String) {
    let re = Regex::new(r"[^a-z]").unwrap();
    let row = re.replace_all(&atom, "").to_string();

    let re = Regex::new(r"[a-z]").unwrap();
    let repl: String = re.replace_all(&atom, "").to_string();

    let parts: Vec<&str> = repl.split("_").collect();

    (row, parts[0].to_owned(), parts[1].to_owned())
}

fn is_allowed(left: String, right: String) -> bool {
    let atoms_l = make_atomic(left);
    let atoms_r = make_atomic(right);

    for a_l in &atoms_l {
        let (row, col, val) = deconstruct(a_l);
        for a_r in &atoms_r {
            let (r, c, v) = deconstruct(a_r);

            if (row == r && val == v && col != c)
                || (col == c && val == v && row != r)
                || (row == r && col == c && val != v)
            {
                return false;
            }
        }
    }
    true
}

fn and_clause(left: Vec<String>, right: Vec<String>) -> Vec<String> {
    let variants: Vec<Vec<String>> = iproduct!(left, right).map(|(a, b)| vec![a, b]).collect();
    let mut finished: Vec<String> = Vec::new();
    for mut variant in variants {
        if variant.contains(&"False".to_owned()) {
            variant = vec!["False".to_owned()];
        } else if variant.contains(&"True".to_owned()) {
            variant.retain(|x| x != &"True".to_owned());
        } else if !!!is_allowed(variant[0].to_string(), variant[1].to_string()) {
            continue;
        }

        if variant.len() > 1 {
            finished.push(variant.join(" & "));
        } else {
            finished.push(variant[0].to_string());
        }
    }
    let set: HashSet<_> = finished.drain(..).collect();
    finished.extend(set.into_iter());
    finished
}

fn simple_and(mut ks: Vec<String>) -> String {
    let mut set: HashSet<_> = ks.drain(..).collect();
    set.remove("True");
    ks.extend(set.into_iter());
    if ks.len() == 0 {
        return "True".to_owned();
    }
    if ks.len() > 1 {
        return ks.join(" & ");
    }
    return ks[0].to_owned();
}

fn or_clause(mut ks: Vec<String>) -> String {
    if ks.len() == 0 {
        return "ERR".to_owned();
    }
    let mut set: HashSet<_> = ks.drain(..).collect();
    set.remove("False");
    ks.extend(set.into_iter());

    let mut clause: Vec<String> = Vec::new();
    for k in &ks {
        if is_atomic(k) || k.starts_with("(") {
            clause.push(k.to_string());
        } else {
            clause.push(format!("( {} )", k));
        }
    }
    return clause.iter().join(" | ");
}

fn grouped(clause: String) -> String {
    if clause.len() == 0 {
        return "".to_owned();
    }
    if is_atomic(&clause) {
        return clause;
    }
    return format!("( {} )", clause);
}

pub fn new_buffer(mut size: u8) -> Vec<Vec<String>> {
    let mut buffer: Vec<_> = Vec::new();
    if size < 2 {
        size = *ORDER.get();
    }
    for _ in 0..size {
        buffer.push(vec![]);
    }
    buffer
}

fn equalize(
    mut left: Vec<Vec<String>>,
    mut right: Vec<Vec<String>>,
) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    while left.len() < right.len() {
        left.push(vec!["False".to_owned()])
    }
    while right.len() < left.len() {
        right.push(vec!["False".to_owned()])
    }
    (left, right)
}

fn reduce(propositions: Vec<Vec<Vec<String>>>, mode: String) -> String {
    let mut output: Vec<_> = Vec::new();
    for truth in &propositions {
        let mut t: Vec<_> = Vec::new();
        for option in truth {
            let mut o: Vec<_> = Vec::new();
            for op in option {
                if !!!op.contains("False") {
                    o.push(op.clone());
                }
            }
            if option.len() > 0 {
                t.push(or_clause(o));
            }
        }
        output.push(grouped(or_clause(t)));
    }

    if mode == "or" {
        or_clause(output)
    } else {
        simple_and(output)
    }
}
