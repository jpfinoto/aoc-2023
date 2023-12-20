use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use tqdm::Iter;

advent_of_code::solution!(20);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum SignalLevel {
    High,
    Low,
}

enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Transmission {
    signal: SignalLevel,
    target: String,
    source: String,
}

#[derive(Debug)]
struct ModuleBase {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl ModuleBase {
    fn new(name: String) -> ModuleBase {
        ModuleBase {
            name,
            inputs: vec![],
            outputs: vec![],
        }
    }
}

trait Module {
    fn handle(&mut self, transmission: Transmission) -> Vec<Transmission>;
    fn base(&self) -> &ModuleBase;
    fn base_mut(&mut self) -> &mut ModuleBase;
    fn inputs(&self) -> &Vec<String> {
        &self.base().inputs
    }
    fn outputs(&self) -> &Vec<String> {
        &self.base().outputs
    }
    fn name(&self) -> &String {
        &self.base().name
    }

    fn broadcast(&self, signal: SignalLevel) -> Vec<Transmission>
    where
        Self: Sized,
    {
        self.outputs()
            .iter()
            .map(|target| Transmission {
                signal,
                target: target.clone(),
                source: self.name().clone(),
            })
            .collect()
    }

    fn connect_input(&mut self, name: String) {
        self.base_mut().inputs.push(name)
    }

    fn connect_output(&mut self, name: String) {
        self.base_mut().outputs.push(name)
    }
}

#[derive(Debug)]
struct BroadcastModule {
    base: ModuleBase,
}

impl BroadcastModule {
    fn new(name: String) -> Self {
        Self {
            base: ModuleBase::new(name),
        }
    }
}

impl Module for BroadcastModule {
    fn handle(&mut self, transmission: Transmission) -> Vec<Transmission> {
        self.broadcast(transmission.signal)
    }

    fn base(&self) -> &ModuleBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ModuleBase {
        &mut self.base
    }
}

#[derive(Debug)]
struct FlipFlopModule {
    base: ModuleBase,
    is_on: bool,
}

impl FlipFlopModule {
    fn new(name: String) -> Self {
        Self {
            base: ModuleBase::new(name),
            is_on: false,
        }
    }
}

impl Module for FlipFlopModule {
    fn handle(&mut self, transmission: Transmission) -> Vec<Transmission> {
        let output = match transmission.signal {
            SignalLevel::High => None,
            SignalLevel::Low => match self.is_on {
                true => {
                    self.is_on = false;
                    Some(SignalLevel::Low)
                }
                false => {
                    self.is_on = true;
                    Some(SignalLevel::High)
                }
            },
        };

        if let Some(broadcast_signal) = output {
            self.broadcast(broadcast_signal)
        } else {
            vec![]
        }
    }

    fn base(&self) -> &ModuleBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ModuleBase {
        &mut self.base
    }
}

#[derive(Debug)]
struct ConjunctionModule {
    base: ModuleBase,
    state: HashMap<String, SignalLevel>,
}

impl ConjunctionModule {
    fn new(name: String) -> Self {
        Self {
            base: ModuleBase::new(name),
            state: HashMap::new(),
        }
    }
}

impl Module for ConjunctionModule {
    fn handle(&mut self, transmission: Transmission) -> Vec<Transmission> {
        *self
            .state
            .entry(transmission.source)
            .or_insert(SignalLevel::Low) = transmission.signal.clone();

        match self.state.values().all_equal_value().ok() {
            None => self.broadcast(SignalLevel::High),
            Some(SignalLevel::High) => self.broadcast(SignalLevel::Low),
            Some(SignalLevel::Low) => self.broadcast(SignalLevel::High),
        }
    }

    fn base(&self) -> &ModuleBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ModuleBase {
        &mut self.base
    }

    fn connect_input(&mut self, name: String) {
        self.state.insert(name.clone(), SignalLevel::Low);
        self.base_mut().inputs.push(name)
    }
}

fn parse_line(line: &str) -> Option<(String, ModuleType, Vec<String>)> {
    let (declaration, connections) = line.split(" -> ").next_tuple()?;

    let (module_type, name) = if declaration.starts_with("%") {
        (ModuleType::FlipFlop, declaration.split_at(1).1)
    } else if declaration.starts_with("&") {
        (ModuleType::Conjunction, declaration.split_at(1).1)
    } else {
        (ModuleType::Broadcaster, declaration)
    };

    Some((
        name.into(),
        module_type,
        connections
            .split(",")
            .map(str::trim)
            .map(|s| s.into())
            .collect(),
    ))
}

type ModuleMap = HashMap<String, Box<dyn Module>>;
fn build(input: &str) -> ModuleMap {
    let decl = input.split("\n").flat_map(parse_line).collect_vec();
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

    for (name, module_type, _) in &decl {
        let new_module: Box<dyn Module> = match module_type {
            ModuleType::Broadcaster => Box::new(BroadcastModule::new(name.clone())),
            ModuleType::FlipFlop => Box::new(FlipFlopModule::new(name.clone())),
            ModuleType::Conjunction => Box::new(ConjunctionModule::new(name.clone())),
        };

        modules.insert(name.clone(), new_module);
    }

    for (name, _, connections) in &decl {
        for c in connections {
            modules.get_mut(name).unwrap().connect_output(c.clone());
            if let Some(output_module) = modules.get_mut(c) {
                output_module.connect_input(name.clone());
            }
        }
    }

    modules
}

fn step(modules: &mut ModuleMap, input: Transmission) -> (i64, i64, Vec<Transmission>) {
    let mut pending = VecDeque::from([input]);
    let mut total_low = 0i64;
    let mut total_high = 0i64;
    let mut unhandled_transmissions = vec![];

    while let Some(t) = pending.pop_front() {
        match t.signal {
            SignalLevel::High => total_high += 1,
            SignalLevel::Low => total_low += 1,
        }

        if let Some(target) = modules.get_mut(&t.target) {
            let output = target.handle(t);
            pending.extend(output);
        } else {
            unhandled_transmissions.push(t);
        }
    }

    (total_low, total_high, unhandled_transmissions)
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut modules = build(input);
    let mut total_low = 0i64;
    let mut total_high = 0i64;

    for _ in 0..1000 {
        let (new_low, new_high, _) = step(
            &mut modules,
            Transmission {
                signal: SignalLevel::Low,
                target: "broadcaster".to_string(),
                source: "button".to_string(),
            },
        );
        total_low += new_low;
        total_high += new_high;
    }

    println!("Low: {total_low}, high: {total_high}");

    Some(total_low * total_high)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut modules = build(input);

    for i in (0i64..).tqdm() {
        let (_, _, unhandled) = step(
            &mut modules,
            Transmission {
                signal: SignalLevel::Low,
                target: "broadcaster".to_string(),
                source: "button".to_string(),
            },
        );

        if unhandled
            .iter()
            .any(|t| t.target == "rx" && t.signal == SignalLevel::Low)
        {
            // doesn't work lol
            println!("Broke at {i}");
            break;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
