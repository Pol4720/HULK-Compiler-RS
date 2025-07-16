use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Production {
    pub lhs: String,
    pub rhs: Vec<String>,
}

pub fn load_grammar(path: &str) -> Vec<Production> {
    let file = File::open(path).expect("Cannot open grammar file");
    let reader = BufReader::new(file);

    let mut productions = Vec::new();
    let mut last_lhs = String::new();

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if let Some(pos) = line.find("->") {
            let lhs = line[..pos].trim().to_string();
            let rhs_all = line[pos + 2..].trim();

            for alt in rhs_all.split('|') {
                let rhs: Vec<String> = alt.trim()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                productions.push(Production { lhs: lhs.clone(), rhs });
            }

            last_lhs = lhs;
        } else if let Some(pos) = line.find('|') {
            let rhs = line[pos + 1..].trim();
            let rhs: Vec<String> = rhs.split_whitespace().map(|s| s.to_string()).collect();
            productions.push(Production { lhs: last_lhs.clone(), rhs });
        }
    }

    productions
}
