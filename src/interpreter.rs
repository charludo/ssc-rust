use crate::append_prefills;
use crate::base_rules::ROWS;
use crate::display;
use crate::ORDER;
use regex::Regex;
// use std::fs;
// use std::io::prelude::*;
use std::iter::repeat;
// use std::process::Command;

pub fn solve(output: String, i: u32) -> bool {
    if output.contains("UNSATISFIABLE formula") {
        match i {
            0 => display(format!("Sudoku is unsatisfiable.")),
            1 => display(format!(
                "Sudoku is uniquely solvable. No further solutions exist."
            )),
            _ => display(format!(
                "No further solutions exist. Total number of solutions: {}",
                i
            )),
        }
        return false;
    }

    let (solution, prefills) = extract_solution(output);
    display(format!("Solution #{}:", i + 1));
    prettify(solution);

    let prefills = format!(" & !({})", prefills.join(" & "));
    append_prefills(prefills);

    return true;
}

// pub fn postprocess(output: &str) {
//     let (solution, _) = extract_solution(output.to_string());
//     prettify(solution);
// }

fn extract_solution(output: String) -> (Vec<String>, Vec<String>) {
    let re = Regex::new(r"\n").unwrap();
    let output: String = re.replace_all(&output, "\n").to_string();

    let mut prefills: Vec<String> = Vec::new();
    let mut solution: Vec<String> = Vec::new();
    for _ in 0..u8::pow(*ORDER.get(), 2) {
        solution.push(".".to_owned());
    }

    let re = Regex::new(r"(?P<i>[a-z]+)(?P<j>\d+)_(?P<k>\d+) = 1").unwrap();
    for caps in re.captures_iter(&output) {
        let i: &str = &caps["i"];
        let j: usize = caps["j"].parse().unwrap();
        let k: &str = &caps["k"];
        let index = ROWS.get().iter().position(|r| r == i).unwrap();

        solution[(j - 1) + index * (*ORDER.get() as usize)] = k.to_string();
        prefills.push(format!("{}{}_{}", i, j, k));
    }

    (solution, prefills)
}

fn prettify(solution: Vec<String>) {
    let c = if *ORDER.get() < 10 { 1 } else { 2 };
    let order = (*ORDER.get() as f32).sqrt() as u8;

    let n_elem = (0..c).map(|_| "-").collect::<String>();
    let a_elem: String = repeat(n_elem)
        .take(order as usize)
        .collect::<Vec<String>>()
        .join("-");
    let border: String = repeat(a_elem)
        .take(order as usize)
        .collect::<Vec<String>>()
        .join(" | ");
    let header: String = format!("     {}", border);

    display(String::new());
    display(format!("{}", header));

    let o: usize = *ORDER.get() as usize;

    for i in 0..o {
        let line = &solution[i * o..(i + 1) * o];
        let mut offset: usize = 0;
        let mut line = line
            .iter()
            .map(|x| {
                if x.len() == c {
                    x.to_owned()
                } else {
                    format!(" {}", x)
                }
            })
            .collect::<Vec<String>>();

        for j in 1..o {
            if j as u8 % order == 0 {
                line.insert(j + offset, " ".to_owned());
                offset += 1;
            }
        }
        let line: String = line.join(" ");
        display(format!("   | {}", line));

        if (0..o - 1)
            .collect::<Vec<usize>>()
            .iter()
            .filter(|j| (*j + 1) % (order as usize) == 0)
            .map(|j| *j)
            .collect::<Vec<usize>>()
            .contains(&i)
        {
            display(format!("   -"));
        }
    }
    display(String::new());
}
