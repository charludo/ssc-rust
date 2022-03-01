use crate::helpers;

pub fn builtin(prefix: &str, args: Vec<Vec<Vec<String>>>) -> Vec<Vec<Vec<String>>> {
    match prefix {
        "!!" => distinct(args),
        "~~" => unique(args),
        _ => unreachable!(),
    }
}

fn distinct(args: Vec<Vec<Vec<String>>>) -> Vec<Vec<Vec<String>>> {
    let mut fields: Vec<Vec<String>> = Vec::new();
    for field in args {
        let mut f: Vec<String> = Vec::new();
        for v in field {
            f.push(v[0].to_string());
        }
        fields.push(f);
    }
    let mut buffer = helpers::new_buffer(0);

    for i in 1..fields.len() {
        let left = &fields[i];
        for j in i + 1..fields.len() {
            let right = &fields[j];

            for v in 0..left.len() {
                buffer[0].push(format!("(!{} | !{})", left[v], right[v]));
            }
        }
    }
    buffer[0] = vec![buffer[0].join(" & ")];
    vec![buffer]
}

fn unique(args: Vec<Vec<Vec<String>>>) -> Vec<Vec<Vec<String>>> {
    let mut fields: Vec<Vec<String>> = Vec::new();
    for field in args {
        let mut f: Vec<String> = Vec::new();
        for v in field {
            f.push(v[0].to_string());
        }
        fields.push(f);
    }
    let mut buffer = helpers::new_buffer(0);

    for field in fields {
        for v1 in 0..field.len() {
            for v2 in v1 + 1..field.len() {
                buffer[0].push(format!("(!{} | !{})", field[v1], field[v2]));
            }
        }
    }
    buffer[0] = vec![buffer[0].join(" & ")];
    vec![buffer]
}
