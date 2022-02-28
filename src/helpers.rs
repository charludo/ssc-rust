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

fn new_buffer(size: u8) -> Vec<Vec<String>> {
    let mut buffer: Vec<_> = Vec::new();
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
