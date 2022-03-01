use crate::helpers;
use itertools::iproduct;
use std::collections::HashSet;

pub fn compare(
    left: Vec<Vec<String>>,
    comparator: &str,
    right: Vec<Vec<String>>,
) -> Vec<Vec<String>> {
    match comparator {
        "=" => eq(left, right),
        "!=" => neq(left, right),
        "<" => lt(left, right, 0),
        "<=" => lt(left, right, 1),
        ">" => gt(left, right, 1),
        ">=" => gt(left, right, 0),
        "||" => por(left, right),
        _ => unreachable!(),
    }
}

fn eq(left: Vec<Vec<String>>, right: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let len_left = left.len();
    let mut buffer = helpers::new_buffer(len_left as u8);
    let (left, right) = helpers::equalize(left, right);

    for i in 0..len_left {
        let options = helpers::and_clause(left[i].to_vec(), right[i].to_vec());
        buffer[i] = if options.len() > 0 {
            options
        } else {
            vec!["False".to_owned()]
        };
    }
    buffer
}

fn neq(left: Vec<Vec<String>>, right: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let len_left = left.len();
    let len_right = right.len();
    let mut buffer = helpers::new_buffer(len_left as u8);
    let (left, right) = helpers::equalize(left, right);

    let combinations: Vec<Vec<usize>> = iproduct!(0..len_left, 0..len_right)
        .filter(|(a, b)| a != b)
        .map(|(a, b)| vec![a, b])
        .collect();

    for comb in combinations {
        let i = comb[0];
        let j = comb[1];
        let mut options = helpers::and_clause(left[i].to_vec(), right[j].to_vec());
        if options.len() > 0 {
            buffer[i].append(&mut options);
        } else {
            buffer[i].append(&mut vec!["False".to_owned()]);
        };
    }
    buffer
}

fn lt(left: Vec<Vec<String>>, right: Vec<Vec<String>>, offset: usize) -> Vec<Vec<String>> {
    let len_right = right.len();
    let mut buffer = helpers::new_buffer(len_right as u8);
    let (left, right) = helpers::equalize(left, right);

    for i in 1 - offset..len_right {
        if right[i] != vec!["False".to_owned()] {
            for j in 0..i + offset {
                let mut options = helpers::and_clause(left[j].to_vec(), right[i].to_vec());
                if options.len() > 0 {
                    buffer[j].append(&mut options);
                } else {
                    buffer[j].append(&mut vec!["False".to_owned()]);
                };
            }
        }
    }
    let mut result: Vec<_> = Vec::new();
    for mut b in buffer {
        let set: HashSet<_> = b.to_vec().drain(..).collect();
        b.extend(set.into_iter());
        result.push(b);
    }
    result
}

fn gt(left: Vec<Vec<String>>, right: Vec<Vec<String>>, offset: usize) -> Vec<Vec<String>> {
    let len_left = right.len();
    let mut buffer = helpers::new_buffer(len_left as u8);
    let (left, right) = helpers::equalize(left, right);

    for i in 0..len_left {
        if right[i] != vec!["False".to_owned()] {
            for j in i + offset..len_left {
                let mut options = helpers::and_clause(left[j].to_vec(), right[i].to_vec());
                if options.len() > 0 {
                    buffer[j].append(&mut options);
                } else {
                    buffer[j].append(&mut vec!["False".to_owned()]);
                };
            }
        }
    }
    let mut result: Vec<_> = Vec::new();
    for mut b in buffer {
        let set: HashSet<_> = b.to_vec().drain(..).collect();
        b.extend(set.into_iter());
        result.push(b);
    }
    result
}

fn por(left: Vec<Vec<String>>, right: Vec<Vec<String>>) -> Vec<Vec<String>> {
    return vec![vec![helpers::reduce(vec![left, right], "or")]];
}
