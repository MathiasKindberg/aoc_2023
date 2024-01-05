//! Part 1:
//! Part 2:

use std::{collections::HashMap, io::BufRead};

type Input = (HashMap<String, Vec<Step>>, Vec<Part>);

fn input() -> Input {
    let stdin = std::io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let mut input = input.split(|row| row.is_empty());

    let workflows = input
        .next()
        .unwrap()
        .iter()
        .map(|flow| {
            let (key, input) = flow.split_once('{').unwrap();
            let input = input.replace('}', "");
            (key.to_owned(), input.split(',').map(Step::new).collect())
        })
        .collect();

    let parts = input
        .next()
        .unwrap()
        .iter()
        .map(|part| Part::new(part))
        .collect();

    (workflows, parts)
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(input: &str) -> Self {
        let part = input.replace(['{', '}'], "");
        let values: Vec<_> = part.split(',').collect();

        // Ugly but works....
        Self {
            x: values[0][2..].parse().unwrap(),
            m: values[1][2..].parse().unwrap(),
            a: values[2][2..].parse().unwrap(),
            s: values[3][2..].parse().unwrap(),
        }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct Step {
    // A step may sometimes not have a condition
    condition: Option<Condition>,
    target: Target,
}

impl Step {
    fn new(condition: &str) -> Self {
        if condition.contains(':') {
            let (operation, target) = condition.split_once(':').unwrap();
            Self {
                condition: Some(Condition {
                    category: Category::new(&operation[0..1]),
                    operator: Operator::new(&operation[1..2]),
                    value: operation[2..].parse().unwrap(),
                }),
                target: Target::new(target),
            }
        } else {
            Self {
                condition: None,
                target: Target::new(condition),
            }
        }
    }

    fn apply(&self, part: &Part) -> WorkflowResult {
        if let Some(condition) = &self.condition {
            let applicable = match condition.category {
                Category::X => condition.operator.compare(part.x, condition.value),
                Category::M => condition.operator.compare(part.m, condition.value),
                Category::A => condition.operator.compare(part.a, condition.value),
                Category::S => condition.operator.compare(part.s, condition.value),
            };

            if applicable {
                WorkflowResult::Applicable(self.target.clone())
            } else {
                WorkflowResult::NotApplicable
            }
        } else {
            WorkflowResult::Applicable(self.target.clone())
        }
    }
}

#[derive(Debug)]
struct Condition {
    category: Category,
    operator: Operator,
    value: usize,
}

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn new(category: &str) -> Self {
        match category {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            other => unreachable!("Unknown category: {other}"),
        }
    }
}

#[derive(Debug)]
enum Operator {
    Greater,
    Less,
}

impl Operator {
    fn new(op: &str) -> Self {
        match op {
            ">" => Operator::Greater,
            "<" => Operator::Less,
            other => unreachable!("Unknown operator: {other}"),
        }
    }

    fn compare(&self, a: usize, b: usize) -> bool {
        match self {
            Self::Greater => a > b,
            Self::Less => a < b,
        }
    }
}

#[derive(Debug, Clone)]
enum WorkflowResult {
    Applicable(Target),
    NotApplicable,
}

// Should likely be renamed
#[derive(Debug, Clone)]
enum Target {
    Accepted,
    Rejected,
    Workflow(String),
}

impl Target {
    fn new(target: &str) -> Self {
        match target {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            target => Self::Workflow(target.to_owned()),
        }
    }
}

fn one(input: Input) {
    let (workflows, parts) = input;
    let now = std::time::Instant::now();

    let mut accepted = 0;
    let mut sum = 0;

    for part in &parts {
        // Todo, should keep on going until done
        let mut next_step = Some(String::from("in"));

        while let Some(workflow) = next_step.take() {
            for step in workflows.get(&workflow).unwrap() {
                // Keep on applying steps until we get a result and then jump to that.
                match step.apply(part) {
                    WorkflowResult::Applicable(result) => {
                        match result {
                            Target::Accepted => {
                                accepted += 1;
                                sum += part.sum();
                            }
                            Target::Rejected => (),
                            Target::Workflow(new_flow) => next_step = Some(new_flow),
                        }
                        break;
                    }
                    WorkflowResult::NotApplicable => (),
                }
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Accepted: {accepted} | Elapsed: {elapsed:?}",);
    println!("Sum: {sum} | Elapsed: {elapsed:?}",);
}

fn two(_input: &[String]) {
    let now = std::time::Instant::now();
    let sum = 0;

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(input);
    // two(&input);
}
