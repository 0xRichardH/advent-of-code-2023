use core::fmt;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

#[derive(Debug)]
enum Module<'a> {
    /// % -> on or off
    FlipFlop(bool),
    /// &
    Conjunction(HashMap<&'a str, Pulse>),
    /// broadcaster
    Broadcaster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct Modules<'a> {
    module: Module<'a>,
    next: Vec<&'a str>,
}

struct Singal<'a> {
    pulse: Pulse,
    from: &'a str,
    to: &'a str,
}

type Configurations<'a> = HashMap<&'a str, Modules<'a>>;

pub fn process_data(input: &str) -> usize {
    let mut configurations = parse_configurations(input);

    let (mut low, mut high) = (0, 0);
    for _ in 0..1000 {
        let (l, h) = press_button(&mut configurations);
        low += l;
        high += h;
    }

    low * high
}

fn parse_configurations(input: &str) -> Configurations {
    let mut conjunctions = Vec::new();
    let mut linked_map = HashMap::new();
    let mut configs: Configurations = input
        .trim()
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split("->").map(|s| s.trim()).collect();
            let modules_str = parts[0];
            let (module, name) = if modules_str == "broadcaster" {
                (Module::Broadcaster, "broadcaster")
            } else {
                let m: Module = modules_str[..1].into();
                let n = &modules_str[1..];
                if let Module::Conjunction(_) = m {
                    conjunctions.push(n);
                }

                (m, n)
            };
            let next: Vec<&str> = parts[1].split(',').map(|s| s.trim()).collect();

            for n in &next {
                linked_map.entry(*n).or_insert(Vec::new()).push(name);
            }

            let modules = Modules { module, next };

            (name, modules)
        })
        .collect();

    for c in conjunctions {
        let Some(modules) = configs.get_mut(c) else {
            continue;
        };
        modules.module.add_linked_modules(linked_map[c].clone());
    }

    configs
}

fn press_button(configs: &mut Configurations) -> (usize, usize) {
    let (mut low, mut high) = (1, 0);
    let start = &configs["broadcaster"];
    let mut q = VecDeque::new();
    for n in &start.next {
        q.push_back(Singal {
            pulse: Pulse::Low,
            from: "broadcaster",
            to: n,
        });
    }

    while let Some(s) = q.pop_front() {
        let Singal { pulse, to, .. } = s;
        match pulse {
            Pulse::High => {
                high += 1;
            }
            Pulse::Low => {
                low += 1;
            }
        }

        let Some(to_configs) = configs.get_mut(to) else {
            continue;
        };
        let Some(pulse) = to_configs.module.receive(s) else {
            continue;
        };

        for n in &configs[to].next {
            let singal = Singal {
                pulse,
                from: to,
                to: n,
            };
            q.push_back(singal);
        }
    }

    (low, high)
}

impl<'a> From<&str> for Module<'a> {
    fn from(s: &str) -> Self {
        match s {
            "%" => Module::FlipFlop(false),
            "&" => Module::Conjunction(HashMap::new()),
            _ => Module::Broadcaster,
        }
    }
}

impl<'a> Module<'a> {
    fn add_linked_modules(&mut self, linked_modules: Vec<&'a str>) {
        if let Module::Conjunction(store) = self {
            store.extend(linked_modules.into_iter().map(|m| (m, Pulse::Low)));
        }
    }

    fn receive(&mut self, singal: Singal<'a>) -> Option<Pulse> {
        let mut pulse = singal.pulse;
        match self {
            Module::FlipFlop(on) => {
                if pulse == Pulse::Low {
                    if *on {
                        *on = false;
                    } else {
                        *on = true;
                        pulse = Pulse::High;
                    }
                } else {
                    return None;
                }
            }
            Module::Conjunction(store) => {
                store
                    .entry(singal.from)
                    .and_modify(|e| *e = pulse)
                    .or_insert(pulse);
                if store.values().all(|p| p == &Pulse::High) {
                    pulse = Pulse::Low;
                } else {
                    pulse = Pulse::High;
                }
            }
            Module::Broadcaster => (),
        }
        Some(pulse)
    }
}

impl Display for Pulse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pulse::High => write!(f, "high"),
            Pulse::Low => write!(f, "low"),
        }
    }
}

impl<'a> fmt::Debug for Singal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} -> {}", self.from, self.pulse, self.to)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "broadcaster -> a, b, c
    %a -> b
    %b -> c
    %c -> inv
    &inv -> a",
        32000000
    )]
    #[case(
        "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        11687500
    )]
    fn test_process_data(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, process_data(input));
    }
}
