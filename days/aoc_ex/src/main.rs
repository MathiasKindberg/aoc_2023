use std::io::BufRead;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| {
            println!("{line:?}");
            line.ok()
        })
        .collect()
}

fn one(input: &[String]) {
    dbg!(&input);
}

fn main() {
    let input = input();
}
