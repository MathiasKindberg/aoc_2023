use std::io::BufRead;

fn input() -> Vec<Vec<u32>> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .filter(|char| char.is_ascii_digit())
                .filter_map(|digit| digit.to_digit(10))
                .collect()
        })
        .collect()
}

fn one(input: &Vec<Vec<u32>>) {
    let mut sum = 0;
    for row in input {
        let Some(first) = row.first().cloned() else {
            continue;
        };
        let Some(last) = row.last().cloned() else {
            continue;
        };

        let num = last + (10 * first);
        sum += num;
    }
    println!("One: {sum}");
}

fn main() {
    let input = input();
    one(&input);
}
