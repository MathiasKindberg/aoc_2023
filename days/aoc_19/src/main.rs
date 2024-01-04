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
        .into_iter()
        .map(|flow| {
            let (key, input) = flow.split_once("{").unwrap();
            let input = input.replace("}", "");
            (
                key.to_owned(),
                input.split(',').map(|step| Step::new(step)).collect(),
            )
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
}

#[derive(Debug)]
struct Step {
    // A step may sometimes not have a condition
    condition: Option<Condition>,
    target: WorkflowResult,
}

impl Step {
    fn new(condition: &str) -> Self {
        if condition.contains(":") {
            let (operation, target) = condition.split_once(":").unwrap();
            Self {
                condition: Some(Condition {
                    category: Category::new(&operation[0..1]),
                    operator: Operator::new(&operation[1..2]),
                    value: operation[2..].parse().unwrap(),
                }),
                target: WorkflowResult::new(target),
            }
        } else {
            Self {
                condition: None,
                target: WorkflowResult::new(condition),
            }
        }
    }

    fn apply(&self, part: &Part) -> WorkflowResult {
        if let Some(condition) = &self.condition {
            // Todo: apply logic
            self.target.clone()
        } else {
            assert!(
                self.target.is_workflow(),
                "Expected workflow target if no condition"
            );
            self.target.clone()
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
}

// Should likely be renamed
#[derive(Debug, Clone)]
enum WorkflowResult {
    Accepted,
    Rejected,
    Workflow(String),
}

impl WorkflowResult {
    fn new(target: &str) -> Self {
        match target {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            target => Self::Workflow(target.to_owned()),
        }
    }

    fn is_workflow(&self) -> bool {
        match self {
            WorkflowResult::Workflow(_) => true,
            _ => false,
        }
    }
}

fn one(input: Input) {
    let (workflows, parts) = input;
    let now = std::time::Instant::now();
    let sum = 0;

    for workflow in &workflows {
        println!("{workflow:?}")
    }

    println!("====");
    let workflow = String::from("in");
    for part in parts {
        // Todo, should keep on going until done
        let flow = workflows.get(&workflow).unwrap();
        println!("{part:?}");
        println!("{flow:?}");

        for step in workflows.get(&workflow).unwrap() {
            // Keep on applying steps until we get a result
            step.apply(&part);
        }

        break;
    }

    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
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
