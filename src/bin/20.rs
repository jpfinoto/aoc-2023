use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use num::integer::lcm;

advent_of_code::solution!(20);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum SignalLevel {
    High,
    Low,
}

#[derive(Debug)]
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
    module_type: ModuleType,
}

impl ModuleBase {
    fn new(name: String, module_type: ModuleType) -> ModuleBase {
        ModuleBase {
            name,
            module_type,
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
            base: ModuleBase::new(name, ModuleType::Broadcaster),
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
            base: ModuleBase::new(name, ModuleType::FlipFlop),
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
            base: ModuleBase::new(name, ModuleType::Conjunction),
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

fn build_module(name: &String, module_type: &ModuleType) -> Box<dyn Module> {
    match module_type {
        ModuleType::Broadcaster => Box::new(BroadcastModule::new(name.clone())),
        ModuleType::FlipFlop => Box::new(FlipFlopModule::new(name.clone())),
        ModuleType::Conjunction => Box::new(ConjunctionModule::new(name.clone())),
    }
}

fn build(input: &str) -> ModuleMap {
    let decl = input.split("\n").flat_map(parse_line).collect_vec();
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

    for (name, module_type, _) in &decl {
        let new_module: Box<dyn Module> = build_module(name, module_type);

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

#[allow(dead_code)]
fn print_mermaid_diagram(modules: &ModuleMap) {
    println!("stateDiagram-v2");
    println!("    classDef flip fill:#faa");
    println!("    classDef conj fill:#afa");
    println!("    classDef rx fill:#00f,color:#fff");
    println!("    class rx rx");

    for module in modules.values() {
        for output in module.outputs() {
            println!("    {} --> {}", module.name(), output);
        }
        match module.base().module_type {
            ModuleType::Broadcaster => {}
            ModuleType::FlipFlop => println!("    class {} flip", module.name()),
            ModuleType::Conjunction => println!("    class {} conj", module.name()),
        }
    }
}

fn get_block_inputs_and_outputs<'a>(
    group: &HashSet<&'a String>,
    modules: &'a ModuleMap,
) -> (Vec<&'a String>, Vec<&'a String>) {
    let inputs = HashSet::from_iter(
        group
            .iter()
            .flat_map(|m| modules.get(*m))
            .map(|m| m.inputs())
            .flatten(),
    );
    let outputs = HashSet::from_iter(
        group
            .iter()
            .flat_map(|m| modules.get(*m))
            .map(|m| m.outputs())
            .flatten(),
    );

    (
        inputs.difference(group).cloned().collect_vec(),
        outputs.difference(group).cloned().collect_vec(),
    )
}

struct SubGroup {
    input_name: String,
    modules: ModuleMap,
}

fn assemble_subgroup(
    modules: &ModuleMap,
    members: HashSet<&String>,
    input_name: &String,
    output_name: &String,
) -> SubGroup {
    let mut new_modules: ModuleMap = modules
        .values()
        .filter(|m| members.contains(m.name()))
        .map(|m| {
            let mut module = build_module(m.name(), &m.base().module_type);

            for conn in m.inputs().iter().filter(|c| members.contains(c)) {
                module.connect_input(conn.clone());
            }

            for conn in m.outputs().iter().filter(|c| members.contains(c)) {
                module.connect_output(conn.clone());
            }

            (module.name().clone(), module)
        })
        .collect();

    new_modules
        .get_mut(output_name)
        .unwrap()
        .connect_output("output".to_string());

    SubGroup {
        input_name: input_name.into(),
        modules: new_modules,
    }
}

fn build_groups(modules: &ModuleMap) -> Vec<SubGroup> {
    let mut groups = vec![];

    for base_mod_id in modules.get("broadcaster").unwrap().outputs() {
        let mut current_group = HashSet::from([base_mod_id]);
        loop {
            let (inputs, outputs) = get_block_inputs_and_outputs(&current_group, &modules);
            if inputs.len() == 1 && outputs.len() == 1 {
                let output_module = current_group
                    .iter()
                    .filter(|m| {
                        modules
                            .get(**m)
                            .unwrap()
                            .outputs()
                            .contains(outputs[0])
                    })
                    .exactly_one()
                    .unwrap();

                groups.push(assemble_subgroup(
                    modules,
                    current_group.clone(),
                    base_mod_id,
                    output_module,
                ));
                break;
            }
            if inputs.len() != 1 {
                current_group.extend(inputs);
            }
            if outputs.len() != 1 {
                current_group.extend(outputs);
            }
            current_group.remove(&"broadcaster".to_string());
        }
    }
    groups
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

    // println!("Low: {total_low}, high: {total_high}");

    Some(total_low * total_high)
}

pub fn part_two(input: &str) -> Option<u64> {
    let modules = build(input);

    // print_mermaid_diagram(&modules);

    let mut groups = build_groups(&modules);
    let mut cycle = vec![];

    for g in &mut groups {
        // print_mermaid_diagram(&g.modules);
        for i in 0u64.. {
            let (_, _, unhandled) = step(
                &mut g.modules,
                Transmission {
                    signal: SignalLevel::Low,
                    target: g.input_name.clone(),
                    source: "broadcaster".to_string(),
                },
            );
            if unhandled.iter().any(|t| t.signal == SignalLevel::High) {
                cycle.push(i + 1);
                break;
            }
        }
    }

    cycle.into_iter().reduce(lcm)
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
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
