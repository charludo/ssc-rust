use regex::Regex;

fn is_atomic(clause: String) -> bool {
    if clause.contains("&") | clause.contains("|") {
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

fn deconstruct(atom: String) -> (String, String, String) {
    let re = Regex::new(r"[^a-z]").unwrap();
    let row = re.replace_all(&atom, "").to_string();

    let re = Regex::new(r"[a-z]").unwrap();
    let repl: String = re.replace_all(&atom, "").to_string();

    let parts: Vec<&str> = repl.split("_").collect();

    (row, parts[0].to_owned(), parts[1].to_owned())
}
