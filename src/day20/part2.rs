use core::fmt;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use num::Integer;

const END_MOUDLE: &str = "rx";

#[derive(Debug, Clone)]
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
    let (mut configurations, mut cycle_modules_counter, feed) = parse_configurations(input);

    let mut cycle: HashMap<&str, usize> = HashMap::new();
    let mut counter = 0;
    loop {
        counter += 1;
        if press_button(
            &mut configurations,
            &mut cycle_modules_counter,
            &mut cycle,
            feed,
            counter,
        ) {
            dbg!(&cycle);
            return cycle.values().fold(1, |acc, n| acc.lcm(n));
        }
    }
}

fn parse_configurations(input: &str) -> (Configurations, HashMap<&str, usize>, &str) {
    let mut conjunctions = Vec::new();
    let mut linked_map = HashMap::new();
    let mut feed = None;
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
                if *n == END_MOUDLE {
                    feed = Some((name, module.clone()));
                }
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

    // the feed is linked to rx (END_MOUDLE)
    // and it should be conjunction
    let Some(feed) = feed else {
        unreachable!();
    };
    let (feed, Module::Conjunction(_)) = feed else {
        unreachable!();
    };

    let Some(cycle_modules) = linked_map.get(feed) else {
        unreachable!();
    };
    for c in cycle_modules {
        let Some(modules) = configs.get_mut(c) else {
            unreachable!();
        };

        let Module::Conjunction(_) = modules.module else {
            unreachable!();
        };
    }
    let cycle_modules_counter: HashMap<&str, usize> =
        cycle_modules.iter().map(|c| (*c, 0)).collect();

    (configs, cycle_modules_counter, feed)
}

fn press_button<'a>(
    configs: &mut Configurations<'a>,
    cycle_modules_counter: &mut HashMap<&'a str, usize>,
    cycle: &mut HashMap<&'a str, usize>,
    feed: &str,
    counter: usize,
) -> bool {
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
        let to = s.to;

        if to == feed && s.pulse == Pulse::High {
            cycle_modules_counter.entry(s.from).and_modify(|c| *c += 1);

            if cycle.get(s.from).is_none() {
                cycle.insert(s.from, counter);
            }

            if cycle_modules_counter.values().all(|c| *c > 0) {
                return true;
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

    false
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
