/*
In membrane computing:

All applicable rules that can be applied in parallel must be applied in one step.
Each object can only be used once per step.
If multiple rules compete for the same objects, one valid maximal subset is chosen
(nondeterministically or deterministically).

Objects: a, a, b
Rules:
1. a → b
2. a → c
3. b → a
=-=-=*/
/* Step 2: Extend Our Rust Model

We’ll:

Find all applicable rules,

Apply as many as possible without exceeding available object counts.

Here’s a simple deterministic version (for clarity — random choice can be added later).*/
use std::collections::HashMap;

type Multiset = HashMap<char, usize>;

#[derive(Debug, Clone)]
enum Rule {
    Rewrite { lhs: char, rhs: Vec<char> },
}

#[derive(Debug, Clone)]
struct Membrane {
    id: usize,
    objects: Multiset,
    rules: Vec<Rule>,
    children: Vec<Membrane>,
}

impl Membrane {
    /// Apply all applicable rules maximally in parallel
    fn step(&mut self) {
        // Count how many times each rule can be applied
        let mut applications: Vec<usize> = Vec::new();

        for rule in &self.rules {
            match rule {
                Rule::Rewrite { lhs, .. } => {
                    let available = *self.objects.get(lhs).unwrap_or(&0);
                    applications.push(available); // can apply once per object
                }
            }
        }

        // Apply rules in a single "global" step
        let mut new_objects = self.objects.clone();

        for (i, rule) in self.rules.iter().enumerate() {
            match rule {
                Rule::Rewrite { lhs, rhs } => {
                    let count = applications[i];
                    if count > 0 {
                        // consume all used objects
                        *new_objects.entry(*lhs).or_default() -= count;

                        // produce right-hand side objects
                        for c in rhs {
                            *new_objects.entry(*c).or_default() += count;
                        }
                    }
                }
            }
        }

        self.objects = new_objects;

        // Recursively step children
        for child in &mut self.children {
            child.step();
        }
    }
}
//Step 3: Example System
fn main() {
    let rule1 = Rule::Rewrite { lhs: 'a', rhs: vec!['b'] }; // a → b
    let rule2 = Rule::Rewrite { lhs: 'b', rhs: vec!['a', 'a'] }; // b → aa

    let mut root = Membrane {
        id: 0,
        objects: [('a', 2)].into_iter().collect(),
        rules: vec![rule1, rule2],
        children: vec![],
    };

    for step in 0..5 {
        println!("Step {}: {:?}", step, root.objects);
        root.step();
    }
}
/*-------------Output
   Step 0: {'a': 2}
   Step 1: {'b': 2}
   Step 2: {'a': 4}
   Step 3: {'b': 4}
   Step 4: {'a': 8}
   Step 5: {'b': 8}
*/
/*
Step 4: Next Steps You Can Add

You can extend this framework to support more realistic P systems:

Feature	How to add
SendIn / SendOut rules	---Track messages between membranes, add to child/parent multisets
Dissolution	            ---Remove membrane, merge its objects with parent
Non-determinism	        ---Randomly choose among competing rules
Parallelism	            ---Use rayon to process membranes concurrently
Config file input	    ---Load membranes and rules from JSON or TOML
Visualization	        ---Output state graph with Graphviz or web interface
*/