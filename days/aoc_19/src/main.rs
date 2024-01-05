//! Part 1:
//! Part 2:

use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Clone)]
struct Input(HashMap<String, Vec<Step>>, Vec<Part>);

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

    Input(workflows, parts)
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct PartRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl PartRange {
    fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }
}

#[derive(Debug, Clone)]
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

    /// Applies range and returns result.
    fn apply_range(&self, part_range: &PartRange) -> RangeResult {
        println!("Applying range: {part_range:?}");

        let mut applicable_range = part_range.clone();
        let mut not_applicable_range = part_range.clone();

        if let Some(condition) = &self.condition {
            println!("Condition: {condition:?}");
            let (applicable, not_applicable) = match condition.category {
                Category::X => {
                    let (applicable, not_applicable) = condition
                        .operator
                        .compare_range(part_range.x, condition.value);

                    if let Some(applicable) = applicable {
                        applicable_range.x = applicable;
                    }

                    if let Some(not_applicable) = not_applicable {
                        not_applicable_range.x = not_applicable;
                    }

                    (
                        applicable.map(|_| applicable_range),
                        not_applicable.map(|_| not_applicable_range),
                    )
                }
                Category::M => {
                    let (applicable, not_applicable) = condition
                        .operator
                        .compare_range(part_range.m, condition.value);

                    if let Some(applicable) = applicable {
                        applicable_range.m = applicable;
                    }

                    if let Some(not_applicable) = not_applicable {
                        not_applicable_range.m = not_applicable;
                    }

                    (
                        applicable.map(|_| applicable_range),
                        not_applicable.map(|_| not_applicable_range),
                    )
                }
                Category::A => {
                    let (applicable, not_applicable) = condition
                        .operator
                        .compare_range(part_range.a, condition.value);

                    if let Some(applicable) = applicable {
                        applicable_range.a = applicable;
                    }

                    if let Some(not_applicable) = not_applicable {
                        not_applicable_range.a = not_applicable;
                    }

                    (
                        applicable.map(|_| applicable_range),
                        not_applicable.map(|_| not_applicable_range),
                    )
                }
                Category::S => {
                    let (applicable, not_applicable) = condition
                        .operator
                        .compare_range(part_range.s, condition.value);

                    if let Some(applicable) = applicable {
                        applicable_range.s = applicable;
                    }

                    if let Some(not_applicable) = not_applicable {
                        not_applicable_range.s = not_applicable;
                    }

                    (
                        applicable.map(|_| applicable_range),
                        not_applicable.map(|_| not_applicable_range),
                    )
                }
                _ => todo!(),
            };

            println!("Applicable: {applicable:?}\nNot Applicable: {not_applicable:?}");

            RangeResult::Split(Split {
                applicable: applicable.map(|applicable| (applicable, self.target.clone())),
                not_applicable,
            })
        } else {
            // If no condition send the range to the next one.
            RangeResult::Kept(self.target.clone())
        }
    }
}

#[derive(Debug, Clone)]
enum RangeResult {
    Split(Split),
    Kept(Target),
}

#[derive(Debug, Clone)]
struct Split {
    //Jumps to another workflow
    applicable: Option<(PartRange, Target)>,
    // Goes to the next step
    not_applicable: Option<PartRange>,
}

#[derive(Debug, Clone)]
struct Condition {
    category: Category,
    operator: Operator,
    value: usize,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

    fn compare_range(
        &self,
        a: (usize, usize),
        target: usize,
        //         Applicable           Not applicable
    ) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
        println!("Range: {a:?} {self:?} {target}");
        match self {
            Self::Greater => {
                let applicable = is_positive_range((a.0.max(target), a.1));
                let not_applicable = is_positive_range((a.0, a.1.min(target - 1)));

                (applicable, not_applicable)
            }
            Self::Less => {
                let applicable = is_positive_range((a.0, a.1.min(target - 1)));
                let not_applicable = is_positive_range((a.0.max(target), a.1));

                (applicable, not_applicable)
            }
        }
    }
}

// Returns Some(range) if the range exists.
fn is_positive_range(range: (usize, usize)) -> Option<(usize, usize)> {
    // If lower end is larger than upper end this range do not exist anymore.
    if range.0 > range.1 {
        None
    } else {
        Some(range)
    }
}

#[derive(Debug, Clone)]
enum WorkflowResult {
    Applicable(Target),
    NotApplicable,
}

// Should likely be renamed
#[derive(Debug, Clone, PartialEq, Eq)]
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

fn one(input: &Input) {
    let now = std::time::Instant::now();
    let Input(workflows, parts) = input;

    let mut accepted = 0;
    let mut sum = 0;

    for part in parts {
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
    println!("Sum: {sum} | Elapsed: {elapsed:?}\n",);
}
/// 1.
/// Idea: Work backwards utilizing ranges. Start from all steps that end up in "A"
/// Some kind of memoization to know when we converge on known paths.
/// SCRATCH TAHT!
///
/// 2.
/// Work forwards from "in" with ranges.
/// Whenever encountering a condition take both pats, either having the value lower
/// or being above and taking the next choice.
fn two(input: &Input) {
    let now = std::time::Instant::now();
    let Input(workflows, _) = input;

    let sum = 0;

    // Todo, should keep on going until done

    let part_range = PartRange::new();
    let mut queue = vec![String::from("in")];
    while let Some(workflow) = queue.pop() {
        let workflow = workflows.get(&workflow).unwrap();
        println!("\n===================");
        println!("Workflow: {workflow:?}");

        // for step in workflows.get(&workflow).unwrap() {
        for step in workflow {
            match step.apply_range(&part_range) {
                RangeResult::Split(split) => println!("SPLIT: {split:?}"),
                RangeResult::Kept(kept) => println!("KEPT: {kept:?}"),
            }
            todo!()
        }

        // Todo:
        // for step in workflows.get(&workflow).unwrap() {
        //     // Keep on applying steps until we get a result and then jump to that.
        //     match step.apply(part) {
        //         WorkflowResult::Applicable(result) => {
        //             match result {
        //                 Target::Accepted => {
        //                     accepted += 1;
        //                     sum += part.sum();
        //                 }
        //                 Target::Rejected => (),
        //                 Target::Workflow(new_flow) => next_step = Some(new_flow),
        //             }
        //             break;
        //         }
        //         WorkflowResult::NotApplicable => (),
        //     }
        // }
    }

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
