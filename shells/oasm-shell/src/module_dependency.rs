#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
pub fn topo_sort(mods: &[(String, Vec<String>)]) -> Vec<String> {
    let mut incoming: HashMap<String, usize> = HashMap::new();
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    for (m, ds) in mods {
        incoming.entry(m.clone()).or_default();
        for d in ds { *incoming.entry(m.clone()).or_default() += 1; deps.entry(d.clone()).or_default().push(m.clone()); }
    }
    let mut ready: Vec<String> = incoming.iter().filter(|(_, &c)| c == 0).map(|(k, _)| k.clone()).collect();
    let mut order = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    while let Some(n) = ready.pop() {
        if seen.insert(n.clone()) {
            order.push(n.clone());
            if let Some(list) = deps.get(&n) {
                for m in list {
                    if let Some(c) = incoming.get_mut(m) {
                        if *c > 0 { *c -= 1; }
                        if *c == 0 { ready.push(m.clone()); }
                    }
                }
            }
        }
    }
    order
}
