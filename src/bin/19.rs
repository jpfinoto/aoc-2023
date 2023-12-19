use std::collections::HashMap;
use std::iter;
use std::str::FromStr;

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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
