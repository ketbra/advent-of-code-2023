use anyhow::Result;
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::collections::HashMap;
use Category::*;
use Comparison::*;
use PartDecision::*;
use Rule::*;
use Target::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum PartDecision {
    Accepted,
    Rejected,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Target {
    Exit(PartDecision),
    WorkflowName(String),
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Comparison {
    GreaterThan,
    LessThan,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Rule {
    Jump(Target),
    ConditionalJump(Category, Comparison, usize, Target),
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let halves = input.split("\n\n").collect_vec();
    let lines = aoc::parse_list::<String>(halves[0])?;
    let workflows = lines
        .iter()
        .map(|line| parse_workflow(line))
        .map(|workflow| (workflow.name.to_string(), workflow))
        .collect::<HashMap<String, Workflow>>();

    let lines = aoc::parse_list::<String>(halves[1])?;
    let parts = lines.iter().map(|line| parse_part(line)).collect_vec();

    let mut answer = 0;
    for part in parts {
        let decision = run_workflow(&part, &workflows);
        if decision == Accepted {
            answer += part.x + part.m + part.a + part.s;
        }
    }

    Ok(answer)
}

fn run_workflow(part: &Part, workflows: &HashMap<String, Workflow>) -> PartDecision {
    let mut workflow = workflows.get("in").unwrap();

    println!("Part: {part:?}");
    loop {
        for rule in &workflow.rules {
            println!("\\_ In {} considering {rule:?}", workflow.name);
            match rule {
                Jump(Exit(x)) => return x.clone(),
                Jump(WorkflowName(x)) => {
                    workflow = workflows.get(x).unwrap();
                    break;
                }
                ConditionalJump(quality, op, threshold, target) => {
                    let val = match quality {
                        X => part.x,
                        M => part.m,
                        A => part.a,
                        S => part.s,
                    };

                    let matches = match op {
                        GreaterThan => val > *threshold,
                        LessThan => val < *threshold,
                    };

                    if matches {
                        match target {
                            Exit(x) => return x.clone(),
                            WorkflowName(x) => {
                                workflow = workflows.get(x).unwrap();
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn parse_part(line: &str) -> Part {
    let (_, x, m, a, s) = regex_captures!(r#"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$"#, line).unwrap();

    let x = x.parse::<usize>().unwrap();
    let m = m.parse::<usize>().unwrap();
    let a = a.parse::<usize>().unwrap();
    let s = s.parse::<usize>().unwrap();
    Part { x, m, a, s }
}

fn parse_workflow(line: &str) -> Workflow {
    let (_, name, rest) = regex_captures!(r#"([a-z]+)\{(.+)\}$"#, line).unwrap();

    let rules = rest
        .split(',')
        .map(|s| {
            if let Some((_, category, op, threshold, dest)) =
                regex_captures!(r#"^([xmas])([><])(\d+):([a-z]+|[AR])$"#, s)
            {
                let category = match category {
                    "x" => X,
                    "m" => M,
                    "a" => A,
                    "s" => S,
                    o => panic!("Unexpected category, {o}"),
                };
                let op = match op {
                    "<" => LessThan,
                    ">" => GreaterThan,
                    o => panic!("Unexpected operation, {o}"),
                };
                let threshold = threshold.parse::<usize>().unwrap();
                let target = match dest {
                    "A" => Exit(Accepted),
                    "R" => Exit(Rejected),
                    name => WorkflowName(name.to_string()),
                };

                ConditionalJump(category, op, threshold, target)
            } else if let Some((_, decision)) = regex_captures!(r#"^([AR])$"#, s) {
                let part_decision = match decision {
                    "A" => Accepted,
                    "R" => Rejected,
                    o => panic!("Unexpected final decision, {o}"),
                };
                Jump(Exit(part_decision))
            } else if let Some((_, dest)) = regex_captures!(r#"^([a-z]+)$"#, s) {
                Jump(WorkflowName(dest.to_string()))
            } else {
                panic!("Failed to parse workflow line, {s}");
            }
        })
        .collect_vec();

    Workflow {
        name: name.to_string(),
        rules,
    }
}

fn tests() -> anyhow::Result<()> {
    let input = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    let solution = solve(input)?;

    assert_eq!(solution, 19114);

    Ok(())
}
