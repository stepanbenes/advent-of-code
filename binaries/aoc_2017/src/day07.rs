use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use regex::Regex;
use solver::SolverBase;

type NodeRef = Rc<RefCell<Node>>;
type WeakNodeRef = Weak<RefCell<Node>>; // Weak reference to avoid cycles

struct Node {
    name: &'static str,
    #[allow(dead_code)]
    weight: u32,
    parent: Option<WeakNodeRef>, // Weak reference to parent
    children: Vec<NodeRef>,      // Strong references to children
}

impl Node {
    fn new(name: &'static str, weight: u32) -> NodeRef {
        Rc::new(RefCell::new(Node {
            name,
            weight,
            parent: None,
            children: Vec::new(),
        }))
    }

    fn add_child(parent: &NodeRef, child: NodeRef) {
        child.borrow_mut().parent = Some(Rc::downgrade(parent)); // Set parent as Weak reference
        parent.borrow_mut().children.push(child);
    }

    #[allow(dead_code)]
    fn get_parent(&self) -> Option<NodeRef> {
        self.parent.as_ref()?.upgrade() // Upgrade Weak to Rc if still valid
    }
}

#[derive(Debug)]
struct TowerInfo {
    name: &'static str,
    weight: u32,
    children: Vec<&'static str>,
}

pub struct Solver {
    input: Vec<TowerInfo>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let re =
            Regex::new(r"(?P<name>\w+) \((?P<weight>\d+)\)(?: -> (?P<children>[\w, ]+))?").unwrap();
        let mut tower_info = Vec::new();
        for line in input.lines() {
            if let Some(caps) = re.captures(line) {
                let name = caps.name("name").unwrap().as_str();
                let weight: u32 = caps.name("weight").unwrap().as_str().parse().unwrap();
                let children = caps
                    .name("children")
                    .map(|m| m.as_str().split(", ").collect())
                    .unwrap_or_else(Vec::new);

                tower_info.push(TowerInfo {
                    name,
                    weight,
                    children,
                });
            }
        }
        Solver { input: tower_info }
    }

    fn build_tree(&self) -> NodeRef {
        let mut map = HashMap::new();
        for info in self.input.iter() {
            let node = Node::new(info.name, info.weight);
            map.insert(info.name, node);
        }
        for info in self.input.iter() {
            let parent = map.get(info.name).unwrap();
            for child_name in info.children.iter() {
                let child = map.get(child_name).unwrap();
                Node::add_child(parent, child.clone());
            }
        }
        map.values()
            .find(|x| x.borrow().parent.is_none())
            .unwrap()
            .clone()
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let root = self.build_tree();
        root.borrow().name.to_owned()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        7
    }

    fn description(&self) -> &'static str {
        "Program towers"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)",
        )
        .solve_part_one();
        assert_eq!(result, "tknk");
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
