use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

lazy_static! {
    static ref DATA: Vec<Rule> = {
        fs::read_to_string("input/day7/rules.txt")
            .expect("could not open file")
            .lines()
            .map(Rule::parse)
            .collect()
    };
    static ref RULES: HashMap<String, &'static Rule> = {
        let mut rules = HashMap::new();
        for rule in DATA.iter() {
            rules.insert(rule.color.to_string(), rule);
        }
        rules
    };
}

pub fn solve() {
    println!("----- part 1 -----");
    let containers = containing_closure("shiny gold");
    println!("containers: {}", containers.len());
    println!();

    println!("----- part 2 -----");
    let bag_count = count_bags("shiny gold");
    println!("bag count: {}", bag_count);
    println!();
}

#[derive(Debug)]
struct Rule {
    color: String,
    products: Vec<Product>,
}

#[derive(Debug)]
struct Product {
    color: String,
    count: usize,
}

impl Rule {
    fn parse(spec: &str) -> Rule {
        let spec_sides: Vec<String> = spec.split(" bags contain ").map(str::to_string).collect();

        let color = spec_sides[0].to_string();

        let products: Vec<Product> = spec_sides[1]
            .trim_end_matches(".")
            .split(", ")
            .map(Product::parse)
            .collect();

        Rule { color, products }
    }

    fn has_product(&self, color: &str) -> bool {
        self.products.iter().any(|p| p.color == color)
    }
}

impl Product {
    fn parse(spec: &str) -> Product {
        if spec == "no other bags" {
            return Product {
                color: "".to_string(),
                count: 0,
            };
        }

        let spec_parts: Vec<String> = spec.split_ascii_whitespace().map(str::to_string).collect();
        let color = format!("{} {}", spec_parts[1], &spec_parts[2]);
        let count: usize = spec_parts[0].parse().unwrap();

        Product { color, count }
    }
}

fn containing(color: &str) -> HashSet<&str> {
    let containers = DATA
        .iter()
        .filter(move |rule| rule.has_product(color))
        .map(|rule| &rule.color as &str);
    HashSet::from_iter(containers)
}

fn containing_closure(color: &str) -> HashSet<&str> {
    let containers = containing(color);
    let mut others = HashSet::new();
    for container in &containers {
        others = HashSet::from_iter(others.union(&containing_closure(container)).cloned());
    }

    HashSet::from_iter(containers.union(&others).cloned())
}

fn count_bags(color: &str) -> usize {
    let rule = RULES[color];

    rule.products
        .iter()
        .map(|product| {
            if product.count == 0 {
                0
            } else {
                product.count + product.count * count_bags(&product.color)
            }
        })
        .sum()
}
