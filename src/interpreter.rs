use crate::base_rules::ROWS;
use crate::ORDER;
use regex::Regex;
use std::fs;
use std::io::prelude::*;
use std::iter::repeat;
use std::process::Command;

pub fn solve(path: &str) {
    let limboole: &str = "/home/charlotte/bachelorarbeit/limboole1.2/limboole";
    let mut satisfiable: bool = true;
    let mut i: u32 = 0;
    while satisfiable {
        let output = Command::new(limboole)
            .arg("-s")
            .arg(path)
            .output()
            .expect("limboole error");
        let output = String::from_utf8_lossy(&output.stdout).to_string();

        if output.contains("UNSATISFIABLE formula") {
            satisfiable = false;
            continue;
        }

        let (solution, prefills) = extract_solution(output);
        i += 1;
        println!("Solution #{}:", i);
        prettify(solution);

        let prefills = format!(" & !({})", prefills.join(" & "));

        let mut file = fs::OpenOptions::new().append(true).open(path).unwrap();

        if let Err(e) = writeln!(file, "{}", prefills) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    match i {
        0 => println!("Sudoku is unsatisfiable."),
        1 => println!("Sudoku is uniquely solvable. No further solutions exist."),
        _ => println!(
            "No further solutions exist. Total number of solutions: {}",
            i
        ),
    }
}

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

    println!();
    println!("{}", header);

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
        println!("   | {}", line);

        if (0..o - 1)
            .collect::<Vec<usize>>()
            .iter()
            .filter(|j| (*j + 1) % (order as usize) == 0)
            .map(|j| *j)
            .collect::<Vec<usize>>()
            .contains(&i)
        {
            println!("   -");
        }
    }
    println!();
}
