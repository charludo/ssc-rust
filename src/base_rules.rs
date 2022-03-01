use crate::ORDER;
extern crate state;

static KS: state::Storage<Vec<u8>> = state::Storage::new();
pub static ROWS: state::Storage<Vec<String>> = state::Storage::new();
static COLS: state::Storage<Vec<String>> = state::Storage::new();
static ORD: state::Storage<u8> = state::Storage::new();

fn and_clause(ks: Vec<String>) -> String {
    return ks.join(" & ");
}

fn or_clause(ks: Vec<String>) -> String {
    return ks.join(" | ");
}

fn grouped(clause: String) -> String {
    return format!("({})", clause);
}

pub fn get_base_rules() -> String {
    println!("the actual order is: {}", *ORDER.get());
    KS.set((1..=*ORDER.get()).collect());
    let mut rows: Vec<String> = Vec::new();
    for i in 0..*ORDER.get() {
        if i < 26 {
            rows.push(((i + 97) as char).to_string());
        } else {
            rows.push(format!(
                "{}{}",
                ((i / 26) as char),
                (((i % 26) + 97) as char)
            ));
        }
    }
    ROWS.set(rows);

    let mut cols: Vec<String> = Vec::new();
    for i in 0..*ORDER.get() {
        cols.push((i + 1).to_string());
    }
    COLS.set(cols);
    ORD.set((*ORDER.get() as f32).sqrt() as u8);

    let mut rows: Vec<String> = Vec::new();
    for i in ROWS.get() {
        rows.push(row(i));
    }
    let rows: String = and_clause(rows);

    let mut cols: Vec<String> = Vec::new();
    for j in COLS.get() {
        cols.push(col(j));
    }
    let cols: String = and_clause(cols);

    let mut areas: Vec<String> = Vec::new();
    for a in KS.get() {
        areas.push(area(*a as usize));
    }
    let areas: String = and_clause(areas);

    and_clause(vec![number_everywhere(), rows, cols, areas])
}

fn number_everywhere() -> String {
    let mut result: Vec<String> = Vec::new();
    for i in ROWS.get() {
        for j in COLS.get() {
            let mut clause: Vec<String> = Vec::new();
            for k in KS.get() {
                clause.push(format!("{}{}_{}", i, j, k));
            }
            result.push(grouped(or_clause(clause)));
        }
    }
    and_clause(result)
}

fn row(i: &String) -> String {
    let mut result: Vec<String> = Vec::new();
    for j1 in COLS.get() {
        for j2 in COLS.get() {
            if j1 < j2 {
                let mut clause: Vec<String> = Vec::new();
                for k in KS.get() {
                    clause.push(format!("(!{}{}_{} | !{}{}_{})", i, j1, k, i, j2, k));
                }
                result.push(grouped(or_clause(clause)));
            }
        }
    }
    and_clause(result)
}

fn col(j: &String) -> String {
    let mut result: Vec<String> = Vec::new();
    for i1 in ROWS.get() {
        for i2 in ROWS.get() {
            if i1 < i2 {
                let mut clause: Vec<String> = Vec::new();
                for k in KS.get() {
                    clause.push(format!("(!{}{}_{} | !{}{}_{})", i1, j, k, i2, j, k));
                }
                result.push(grouped(or_clause(clause)));
            }
        }
    }
    and_clause(result)
}

fn area(a: usize) -> String {
    let i_start: usize = ((a - 1) / *ORD.get() as usize) * *ORD.get() as usize + 1;
    let j_start: usize = ((a - 1) % *ORD.get() as usize) * *ORD.get() as usize + 1;
    let iv: Vec<usize> = (i_start..i_start + *ORD.get() as usize).collect();
    let jv: Vec<usize> = (j_start..j_start + *ORD.get() as usize).collect();

    let mut positions: Vec<String> = Vec::new();
    for i in &iv {
        for j in &jv {
            positions.push(format!("{}{}", ROWS.get()[i - 1], j));
        }
    }

    let mut result: Vec<String> = Vec::new();
    for p1 in &positions {
        for p2 in &positions {
            if p1 != p2 {
                let mut clause: Vec<String> = Vec::new();
                for k in KS.get() {
                    clause.push(format!("(!{}_{} | !{}_{})", p1, k, p2, k));
                }
                result.push(grouped(or_clause(clause)));
            }
        }
    }
    and_clause(result)
}
