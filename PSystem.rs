/* In membrane computing, we model computation as:

A hierarchy of membranes (like a tree of cells),

Each membrane contains multisets of objects and rules,

Rules can:

rewrite objects,

send objects to parent/child membranes,

or dissolve membranes.

The system evolves in parallel steps, applying all possible rules simultaneously until halting.

Rust is actually a great language for this because:

Ownership model prevents accidental aliasing,

enum types are ideal for rule variants,

Parallel rule application can use rayon.*/

//Step 1: Define Core Data Structures
use std::collections::HashMap;

/// A multiset of objects (e.g. {'a': 3, 'b': 1})
type Multiset = HashMap<char, usize>;

/// Types of rules in a simple P system
#[derive(Debug, Clone)]
enum Rule {
    /// a → bc (local rewrite)
    Rewrite { lhs: char, rhs: Vec<char> },

    /// a → b↓ (send b to child membrane)
    SendIn { lhs: char, rhs: char, child_id: usize },

    /// a → b↑ (send b to parent membrane)
    SendOut { lhs: char, rhs: char },

    /// a → δ (dissolve membrane)
    Dissolve { lhs: char },
}

/// One membrane with its multiset and rules
#[derive(Debug, Clone)]
struct Membrane {
    id: usize,
    objects: Multiset,
    rules: Vec<Rule>,
    children: Vec<Membrane>,
    dissolved: bool,
}

//Step 2: Apply Rules (Simplified)
impl Membrane {
    fn apply_rules(&mut self) -> Vec<(usize, char)> {
        let mut to_send_up = Vec::new();

        // apply local rules
        for rule in &self.rules {
            match rule {
                Rule::Rewrite { lhs, rhs } => {
                    if let Some(count) = self.objects.get_mut(lhs) {
                        if *count > 0 {
                            *count -= 1;
                            for c in rhs {
                                *self.objects.entry(*c).or_default() += 1;
                            }
                        }
                    }
                }

                Rule::SendOut { lhs, rhs } => {
                    if let Some(count) = self.objects.get_mut(lhs) {
                        if *count > 0 {
                            *count -= 1;
                            to_send_up.push((self.id, *rhs));
                        }
                    }
                }

                Rule::Dissolve { lhs } => {
                    if let Some(count) = self.objects.get_mut(lhs) {
                        if *count > 0 {
                            *count -= 1;
                            self.dissolved = true;
                        }
                    }
                }

                _ => {}
            }
        }

        // recursively apply to children
        for child in &mut self.children {
            to_send_up.extend(child.apply_rules());
        }

        to_send_up
    }
}
// 1)Synchronize between membranes,
// 2)Handle simultaneous application of multiple rules,
//3)Replace or merge dissolved membranes,
//4)Support non-deterministic rule choice.

//Step 3: Parallel Execution Because each membrane is isolated, this parallelization is safe and efficient.
use rayon::prelude::*;

impl Membrane {
    fn parallel_step(&mut self) {
        self.children.par_iter_mut().for_each(|child| child.parallel_step());
        self.apply_rules();
    }
}
//Step 4: Example Usage
fn main() {
    let rule1 = Rule::Rewrite { lhs: 'a', rhs: vec!['a', 'b'] }; // a → ab
    let rule2 = Rule::SendOut { lhs: 'b', rhs: 'c' };             // b → c↑

    let mut root = Membrane {
        id: 0,
        objects: [('a', 1)].into_iter().collect(),
        rules: vec![rule1, rule2],
        children: vec![],
        dissolved: false,
    };

    for step in 0..5 {
        println!("Step {}: {:?}", step, root.objects);
        root.apply_rules();
    }
}
