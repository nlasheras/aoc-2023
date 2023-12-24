use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast
}

#[derive(Clone)]
pub struct Module {
    pub name : String,
    pub mtype : ModuleType,
    pub inputs : HashMap<String, usize>,
    pub outputs : Vec<String>,
    pub state : usize,
}

impl Module {
    fn from(input: &str) -> Module {
        let mut parts = input.split(" -> ");
        let mut name = parts.next().unwrap();
        let mut mtype = ModuleType::Broadcast;
        if name.starts_with('%') {
            mtype = ModuleType::FlipFlop;
            name = &name[1..name.len()];
        } else if name.starts_with('&') {
            mtype = ModuleType::Conjunction;
            name = &name[1..name.len()];
        } 
        let outputs = parts.next().unwrap().split(", ").map(|s| s.to_string()).collect();
        Module{ name: name.to_string(), mtype: mtype, outputs, state: 0, inputs: HashMap::new() }
    }
    
    pub fn simulate(&mut self, input: &String, pulse: usize) -> Vec<(usize, &str)> {
        match self.mtype {
            ModuleType::FlipFlop => {
                if pulse == 0 {
                    self.state = if self.state == 0 { 1 } else { 0 };
                    self.outputs.iter().map(|o| (self.state, o.as_str())).collect()
                }
                else {
                    Vec::new()
                }
            },
            ModuleType::Conjunction => {
                self.inputs.entry(input.clone()).and_modify(|e| *e = pulse);
                let output = if self.inputs.values().all(|v| *v == 1) { 0 } else { 1 };
                self.outputs.iter().map(|o| (output, o.as_str())).collect()
            },
            ModuleType::Broadcast => {
                self.outputs.iter().map(|o| (pulse, o.as_str())).collect()
            }
        }
    }
}

#[derive(Clone)]
pub struct System {
    pub modules : HashMap<String, Module>,
    pub output: Vec<u64>
}

impl System {

    fn push_once(&mut self) {
        let mut pulses = VecDeque::from([("button".to_string(), 0, "broadcaster".to_string())]);
        while !pulses.is_empty() {
            let (from, pulse, to) = pulses.pop_front().unwrap();
            self.output[pulse] += 1;
            if to == "output" {
                continue;
            }
            if let Some(module) = self.modules.get_mut(&to.to_string()) {
                let os = module.simulate(&from, pulse);
                for o in os {
                    pulses.push_back((to.clone(), o.0, o.1.to_string()));
                }
            }
        }
    }
}

#[aoc_generator(day20)]
pub fn parse_input(input: &str) -> System {
    let modules_list = input.lines().map(Module::from).collect::<Vec<Module>>();
    let mut modules = HashMap::new();
    for m in modules_list.iter() {
        modules.insert(m.name.clone(), m.clone());
    }
    for (name, m) in modules.iter_mut() {
        if m.mtype == ModuleType::Conjunction {
            let inputs = modules_list.iter().filter(|m| m.outputs.contains(name)).map(|m| m.name.clone()).collect::<Vec<String>>();
            for s in inputs {
                m.inputs.insert(s.clone(), 0);    
            }
        }
    }
    System { modules: modules, output: vec![0, 0] }
}

#[aoc(day20, part1)]
pub fn test_push1000(input: &System) -> u64 {
    let mut system = input.clone();
    for _ in 0..1000 {
        system.push_once();
    }
    (system.output[0] * system.output[1]) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY20_EXAMPLE1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    
    #[test]
    fn test_day20_part1_example1() {
        let input = parse_input(DAY20_EXAMPLE1);
        assert_eq!(test_push1000(&input), 32000000);
    }

    const DAY20_EXAMPLE2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_day20_part1_example2() {
        let input = parse_input(DAY20_EXAMPLE2);
        assert_eq!(test_push1000(&input), 11687500);
    }

}