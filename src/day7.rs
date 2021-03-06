use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct BagRule {
    pub children: Vec<(usize, String)>,
    pub parents: Vec<String>,
}

type BagRuleSet = HashMap<String, BagRule>;

fn parse_bag_rules(lines: Vec<&str>) -> BagRuleSet {
    let mut ret = BagRuleSet::new();
    let mut edges = Vec::<(String, (usize, String))>::new();

    let bag_pattern = Regex::new("[0-9]+ [a-z]+ [a-z]+").unwrap();

    for line in lines {
        let title_and_rest = line
            .split("bags contain")
            .map(|x| x.trim())
            .collect::<Vec<_>>();

        ret.insert(
            String::from(title_and_rest[0]),
            BagRule {
                children: Vec::new(),
                parents: Vec::new(),
            },
        );

        bag_pattern
            .find_iter(title_and_rest[1])
            .map(|x| {
                let parts = x.as_str().split(" ").collect::<Vec<_>>();
                let count = parts[0].parse::<usize>().unwrap();
                let name = parts[1..].join(" ");

                (count, name)
            })
            .for_each(|child| {
                edges.push((String::from(title_and_rest[0]), child));
            });
    }

    for (parent, (_, child_name)) in &edges {
        ret.get_mut(child_name)
            .unwrap()
            .parents
            .push(parent.clone());
    }

    for (parent, child) in edges {
        ret.get_mut(&parent).unwrap().children.push(child);
    }

    ret
}

fn count_total_parents(rules: &BagRuleSet, name: &str) -> usize {
    let mut visited = HashSet::<String>::new();

    fn recurse(rules: &BagRuleSet, name: &str, visited: &mut HashSet<String>) {
        for parent in &rules[name].parents {
            visited.insert(parent.clone());
            recurse(rules, parent, visited);
        }
    }
    recurse(rules, name, &mut visited);

    visited.len()
}

fn count_total_children(rules: &BagRuleSet, name: &str) -> usize {
    rules[name]
        .children
        .iter()
        .fold(0usize, |acc, (count, name)| {
            acc + count + count * count_total_children(rules, name)
        })
}

pub fn main() {
    let bag_rules = parse_bag_rules(
        std::fs::read_to_string("data/day7.txt")
            .unwrap()
            .lines()
            .collect::<Vec<_>>(),
    );

    let part1 = count_total_parents(&bag_rules, "shiny gold");
    let part2 = count_total_children(&bag_rules, "shiny gold");

    println!("{} {}", part1, part2);
}
