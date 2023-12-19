use std::collections::{HashMap, VecDeque};
use std::fmt::Formatter;
use std::str::FromStr;
use std::{fmt, iter};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(19);

lazy_static! {
    static ref WORKFLOW_RE: Regex = Regex::new(r"^(?P<id>\w+)\{(?P<rules>.*)}$").unwrap();
    static ref RULE_RE: Regex =
        Regex::new(r"(?:(?P<prop>[xmas])(?P<op>[><])(?P<val>\d+):)?(?P<action>\w+)").unwrap();
    static ref PROP_RE: Regex = Regex::new(r"(?P<prop>[xmas])=(?P<val>\d+)").unwrap();
}

#[derive(Debug)]
enum Condition {
    Less(char, i64),
    Greater(char, i64),
    All,
}

#[derive(Debug)]
enum RuleAction {
    Workflow(u64),
    Accept,
    Reject,
}

#[derive(Debug)]
struct Rule {
    condition: Condition,
    action: RuleAction,
}

impl Rule {
    fn from_capture(captures: regex::Captures) -> Option<Rule> {
        let action = match captures.name("action")?.as_str() {
            "A" => RuleAction::Accept,
            "R" => RuleAction::Reject,
            s => RuleAction::Workflow(Workflow::parse_id(s)),
        };
        let prop = captures
            .name("prop")
            .and_then(|m| Some(m.as_str()))
            .and_then(|s| s.chars().next());
        let op = captures.name("op").and_then(|m| Some(m.as_str()));
        let value = captures
            .name("val")
            .and_then(|m| i64::from_str(m.as_str()).ok());

        let condition = match (prop, op, value) {
            (Some(prop), Some(op), Some(value)) => match op {
                ">" => Condition::Greater(prop, value),
                "<" => Condition::Less(prop, value),
                _ => panic!("invalid condition"),
            },
            _ => Condition::All,
        };

        Some(Rule { action, condition })
    }

    fn matches(&self, piece: &Piece) -> bool {
        match self.condition {
            Condition::Less(prop, value) => piece.get_property(prop) < value,
            Condition::Greater(prop, value) => piece.get_property(prop) > value,
            Condition::All => true,
        }
    }

    fn split(&self, range: &PieceRange) -> (Vec<PieceRange>, Vec<(PieceRange, &RuleAction)>) {
        match self.condition {
            Condition::Less(prop, value) => {
                let prop_val_low = range.get_property_low(prop);
                let prop_val_high = range.get_property_high(prop);

                if prop_val_low < value && prop_val_high < value {
                    // both pass
                    (vec![], vec![(*range, &self.action)])
                } else if prop_val_low > value && prop_val_high > value {
                    // neither pass
                    (vec![*range], vec![])
                } else {
                    (
                        vec![range.copy_with_new_lower(prop, value)],
                        vec![(range.copy_with_new_upper(prop, value - 1), &self.action)],
                    )
                }
            }
            Condition::Greater(prop, value) => {
                let prop_val_low = range.get_property_low(prop);
                let prop_val_high = range.get_property_high(prop);

                if prop_val_low > value && prop_val_high > value {
                    // both pass
                    (vec![], vec![(*range, &self.action)])
                } else if prop_val_low < value && prop_val_high < value {
                    // neither pass
                    (vec![*range], vec![])
                } else {
                    (
                        vec![range.copy_with_new_upper(prop, value)],
                        vec![(range.copy_with_new_lower(prop, value + 1), &self.action)],
                    )
                }
            }
            Condition::All => (vec![], vec![(*range, &self.action)]),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    id: u64,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(line: &str) -> Option<Workflow> {
        let captures = WORKFLOW_RE.captures(line)?;
        let id = Self::parse_id(&captures["id"]);
        let rules = RULE_RE
            .captures_iter(&captures["rules"])
            .flat_map(Rule::from_capture)
            .collect_vec();

        Some(Workflow { id, rules })
    }

    fn parse_id(id: &str) -> u64 {
        let bytes: [u8; 8] = id
            .chars()
            .chain(iter::once('-').cycle())
            .map(|c| c as u8)
            .take(8)
            .collect_vec()
            .try_into()
            .unwrap();

        u64::from_be_bytes(bytes)
    }

    fn process(&self, piece: &Piece) -> &RuleAction {
        self.rules
            .iter()
            .filter(|rule| rule.matches(piece))
            .next()
            .map(|rule| &rule.action)
            .expect("no rule matched")
    }

    fn process_range(&self, range: &PieceRange) -> Vec<(PieceRange, &RuleAction)> {
        let mut handled = vec![];
        let mut unhandled = vec![*range];

        for rule in &self.rules {
            let mut new_unhandled = vec![];

            for current_range in &unhandled {
                let (rule_unhandled, rule_handled) = rule.split(current_range);

                new_unhandled.extend_from_slice(&rule_unhandled);
                handled.extend_from_slice(&rule_handled);
            }

            unhandled = new_unhandled;
        }

        assert_eq!(unhandled.len(), 0);

        handled
    }
}

#[derive(Debug)]
struct Piece {
    properties: [i64; 4],
}

impl Piece {
    fn property_index(prop: char) -> usize {
        match prop {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!("invalid property: {prop}"),
        }
    }

    fn get_property(&self, prop: char) -> i64 {
        self.properties[Self::property_index(prop)]
    }

    fn parse(line: &str) -> Option<Piece> {
        let properties = PROP_RE
            .captures_iter(line)
            .map(|cap| i64::from_str(&cap["val"]).unwrap())
            .collect_vec()
            .try_into()
            .ok()?;

        Some(Piece { properties })
    }

    fn process(&self, workflows: &HashMap<u64, &Workflow>, first_workflow: &Workflow) -> bool {
        let mut current_workflow = first_workflow;
        loop {
            match current_workflow.process(self) {
                RuleAction::Workflow(new_id) => current_workflow = &workflows[&new_id],
                RuleAction::Accept => return true,
                RuleAction::Reject => return false,
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct PieceRange {
    from: [i64; 4],
    to: [i64; 4],
}

impl fmt::Display for PieceRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}-{},{}-{},{}-{},{}-{}",
            self.from[0],
            self.to[0],
            self.from[1],
            self.to[1],
            self.from[2],
            self.to[2],
            self.from[3],
            self.to[3],
        ))
    }
}

impl PieceRange {
    fn get_property_low(&self, prop: char) -> i64 {
        self.from[Piece::property_index(prop)]
    }

    fn get_property_high(&self, prop: char) -> i64 {
        self.to[Piece::property_index(prop)]
    }

    fn copy_with_new_lower(&self, prop: char, value: i64) -> PieceRange {
        let mut copy = self.clone();
        copy.from[Piece::property_index(prop)] = value;
        copy
    }

    fn copy_with_new_upper(&self, prop: char, value: i64) -> PieceRange {
        let mut copy = self.clone();
        copy.to[Piece::property_index(prop)] = value;
        copy
    }

    fn combinations(&self) -> i64 {
        (0..4)
            .map(|i| self.to[i] - self.from[i] + 1)
            .reduce(|p, c| p * c)
            .unwrap()
    }
}

fn blocks(input: &str) -> (&str, &str) {
    input
        .split("\n\n")
        .map(str::trim)
        .filter(|b| b.len() > 0)
        .collect_tuple()
        .expect("invalid blocks")
}

pub fn part_one(input: &str) -> Option<i64> {
    let (workflows_block, data_block) = blocks(input);
    let workflow_list = workflows_block
        .split("\n")
        .flat_map(Workflow::parse)
        .collect_vec();

    let workflows = HashMap::from_iter(workflow_list.iter().map(|wf| (wf.id, wf)));
    let first_workflow = workflows[&Workflow::parse_id("in")];

    let pieces = data_block.split("\n").flat_map(Piece::parse).collect_vec();

    let accepted_total = pieces
        .iter()
        .filter(|p| p.process(&workflows, &first_workflow))
        .map(|p| p.properties.iter().sum::<i64>())
        .sum();

    Some(accepted_total)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (workflows_block, _) = blocks(input);
    let workflow_list = workflows_block
        .split("\n")
        .flat_map(Workflow::parse)
        .collect_vec();

    let workflows: HashMap<u64, &Workflow> =
        HashMap::from_iter(workflow_list.iter().map(|wf| (wf.id, wf)));

    let mut queue = VecDeque::from([(
        PieceRange {
            from: [1, 1, 1, 1],
            to: [4000, 4000, 4000, 4000],
        },
        workflows[&Workflow::parse_id("in")],
    )]);

    let mut approved = vec![];

    while let Some((range, wf)) = queue.pop_front() {
        for (new_range, action) in wf.process_range(&range) {
            match action {
                RuleAction::Workflow(id) => queue.push_back((new_range, workflows[id])),
                RuleAction::Accept => approved.push(new_range),
                RuleAction::Reject => {}
            }
        }
    }

    // for r in &approved {
    //     println!("A: {r}");
    // }

    Some(approved.iter().map(PieceRange::combinations).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
