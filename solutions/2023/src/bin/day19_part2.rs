use anyhow::Result;
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::collections::HashMap;
use std::collections::VecDeque;
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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Range {
    min: usize,
    max: usize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn size(&self) -> usize {
        [self.x, self.m, self.a, self.s]
            .iter()
            .map(|r| r.max - r.min + 1)
            .product::<usize>()
    }
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

    let answer = get_accepted_count(
        &PartRange {
            x: Range { min: 1, max: 4000 },
            m: Range { min: 1, max: 4000 },
            a: Range { min: 1, max: 4000 },
            s: Range { min: 1, max: 4000 },
        },
        &workflows,
    );

    Ok(answer)
}

fn get_accepted_count(range: &PartRange, workflows: &HashMap<String, Workflow>) -> usize {
    let mut accepted = 0;

    let mut deque: VecDeque<(PartRange, Workflow)> = VecDeque::new();
    deque.push_back((range.clone(), workflows.get("in").unwrap().clone()));

    while let Some((mut range, workflow)) = deque.pop_front() {
        for rule in &workflow.rules {
            // println!("\\_ In {} considering {rule:?}", workflow.name);
            match rule {
                Jump(Exit(Accepted)) => {
                    accepted += range.size();
                    break;
                }
                Jump(Exit(Rejected)) => {
                    break;
                }
                Jump(WorkflowName(x)) => {
                    deque.push_back((range, workflows.get(x).unwrap().clone()));
                    break;
                }
                ConditionalJump(quality, op, threshold, target) => {
                    let val = match quality {
                        X => range.x,
                        M => range.m,
                        A => range.a,
                        S => range.s,
                    };

                    let skips_full_range = match op {
                        GreaterThan => val.max <= *threshold,
                        LessThan => val.min >= *threshold,
                    };

                    if skips_full_range {
                        continue; // Skip to the next rule
                    }

                    let matches_full_range = match op {
                        GreaterThan => val.min > *threshold,
                        LessThan => val.max < *threshold,
                    };

                    if matches_full_range {
                        match target {
                            Exit(Accepted) => {
                                accepted += range.size();
                                break;
                            }
                            Exit(Rejected) => {
                                break;
                            }
                            WorkflowName(x) => {
                                deque.push_back((range, workflows.get(x).unwrap().clone()));
                                break;
                            }
                        };
                    }

                    // Need to split the range
                    let mut r_match = range.clone();
                    let mut r_reject = range.clone();
                    let (r_match_val, r_reject_val) = match quality {
                        X => (&mut r_match.x, &mut r_reject.x),
                        M => (&mut r_match.m, &mut r_reject.m),
                        A => (&mut r_match.a, &mut r_reject.a),
                        S => (&mut r_match.s, &mut r_reject.s),
                    };
                    match op {
                        Comparison::GreaterThan => {
                            r_match_val.min = threshold + 1;
                            r_reject_val.max = *threshold;
                        }
                        Comparison::LessThan => {
                            r_match_val.max = threshold - 1;
                            r_reject_val.min = *threshold;
                        }
                    };

                    match target {
                        Exit(Accepted) => accepted += r_match.size(),
                        Exit(Rejected) => {}
                        WorkflowName(x) => {
                            deque.push_back((r_match, workflows.get(x).unwrap().clone()));
                        }
                    };

                    range = r_reject;
                }
            }
        }
    }

    accepted
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

    assert_eq!(solution, 167409079868000);

    Ok(())
}
