use std::collections::{HashMap, HashSet};

use regex::Regex;

enum InputStage {
    OrderingRules,
    Updates,
}

fn parse_input() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut input_stage = InputStage::OrderingRules;
    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let re_ordering_rule = Regex::new(r"^(\d+)\|(\d+)").unwrap();
    let re_update = Regex::new(r"\d+").unwrap();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        match input_stage {
            InputStage::OrderingRules => {
                if line.is_empty() {
                    input_stage = InputStage::Updates;
                    continue;
                }
                let captures = re_ordering_rule.captures(&line).unwrap();
                let a = captures.get(1).unwrap().as_str().parse().unwrap();
                let b = captures.get(2).unwrap().as_str().parse().unwrap();
                match ordering_rules.get_mut(&a) {
                    Some(rule) => {
                        rule.push(b);
                    }
                    None => {
                        ordering_rules.insert(a, vec![b]);
                    }
                }
            }
            InputStage::Updates => {
                if line.is_empty() {
                    break;
                }
                let captures: Vec<i32> = re_update
                    .find_iter(&line)
                    .map(|m| m.as_str().parse().unwrap())
                    .collect();
                updates.push(captures);
            }
        }
    }
    (ordering_rules, updates)
}

fn validate_update(ordering_rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    let mut seen_pages: HashSet<i32> = HashSet::new();
    for page in update {
        if let Some(rules) = ordering_rules.get(page) {
            if rules.iter().any(|p| seen_pages.contains(p)) {
                return false;
            }
        }
        seen_pages.insert(*page);
    }
    return true;
}

fn main() {
    let (ordering_rules, updates) = parse_input();
    let mut total = 0;
    for mut update in updates {
        if !validate_update(&ordering_rules, &update) {
            update.sort_by(|a, b| match ordering_rules.get(a) {
                Some(rules) => {
                    if rules.contains(b) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                }
                None => std::cmp::Ordering::Equal,
            });
            total += update[update.len() / 2];
        }
    }
    println!("{}", total);
}
