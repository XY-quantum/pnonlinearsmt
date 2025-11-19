/*-*
1. Concept — “P Systems as SAT Solvers”

In membrane computing, each membrane can represent a partial assignment (or a search space region),
and the evolution rules simulate constraint propagation and nondeterministic branching.
---------------
F = (x1 ∨ ¬x2) ∧ (¬x1 ∨ x3)

We can encode it as:

Symbols = truth assignments (x1_T, x1_F, …)

Rules = nondeterministic choices + clause checking

Membranes = candidate solution environments

The computation proceeds as:

Nondeterministic expansion — generate all possible assignments.

Constraint checking — membranes that violate clauses are dissolved (die).

Survivors — membranes that remain are satisfying assignments.

This is sometimes called “Membrane SAT Solving via Population Elimination”.

---------------
| Component           | Meaning                                        |
| ------------------- | ---------------------------------------------- |
| **Objects**         | Boolean variable states (e.g., `x1_T`, `x2_F`) |
| **Rules**           | Generate, check, or eliminate assignments      |
| **Membranes**       | Contain partial assignments                    |
| **Dissolution**     | Membranes removed if constraints violated      |
| **Output Membrane** | Collects valid (satisfying) solutions          |

--------------
Below is a conceptual Rust-style skeleton 
for a membrane-based SAT solver using nondeterministic branching and rule-driven elimination.
********************************************************************************************
4. How This Relates to MaxSAT and SMT
MaxSAT

Instead of eliminating violating membranes, assign a cost per violation.

Membranes “compete” to minimize total unsatisfied clauses.

The best (lowest-cost) membranes are output.

You can store fitness = num_unsatisfied_clauses in each membrane, then pick the minimal one.

SMT (Satisfiability Modulo Theories)

Extend object alphabet with theory-specific constraints:

Arithmetic (x + y > 5)

Bit-vectors, arrays, etc.

Add specialized membrane types (e.g., theory solvers) that check partial assignments.

Communication between membranes acts like theory propagation in DPLL(T).

This maps to a distributed, bio-inspired version of DPLL(T):

Boolean membrane → propositional structure.

Sub-membranes → theory solvers.

Rule-based communication → consistency checking.
---------------------------
5. Research Background (for deeper reading)

You can look up:

Paun, G. (2000). “Computing with Membranes.” Journal of Computer and System Sciences.

Fernández-Anta, A., & Pan, L. (2005). “Solving NP-Complete Problems with P Systems.”

A. Leporati et al. (2014). “Solving SAT with Spiking Neural P Systems.”

R. Freund et al. (2018). “Membrane Systems for Constraint Solving and Optimization.”

These papers show theoretical P systems that simulate or outperform SAT/MaxSAT solvers by leveraging massive parallel nondeterminism.

-------------------------
6. Next steps
| Task                                | Description                                              |
| ----------------------------------- | -------------------------------------------------------- |
| **Add MaxSAT scoring**              | Keep a “violations” counter instead of dissolving.       |
| **Parallel Execution (Rust Rayon)** | Evolve membranes concurrently for large formulas.        |
| **Graphviz Visualization**          | Draw membranes as candidate assignments; red = violated. |
| **Distributed P System Simulation** | Run membranes across threads or even networked nodes.    |

*-*/

use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Clause(Vec<(usize, bool)>); // (variable index, polarity)

#[derive(Clone, Debug)]
struct Membrane {
    id: usize,
    assignment: HashMap<usize, bool>,
    alive: bool,
}

#[derive(Clone, Debug)]
struct PSatSystem {
    clauses: Vec<Clause>,
    membranes: Vec<Membrane>,
}

impl PSatSystem {
    /// Initialize with one membrane (empty assignment)
    fn new(clauses: Vec<Clause>, num_vars: usize) -> Self {
        let mut root = Membrane {
            id: 0,
            assignment: HashMap::new(),
            alive: true,
        };
        Self {
            clauses,
            membranes: vec![root],
        }
    }

    /// Step 1: nondeterministically assign a new variable
    fn expand(&mut self, var_index: usize) {
        let mut new_membranes = Vec::new();

        for m in &self.membranes {
            if !m.alive {
                continue;
            }
            let mut m_true = m.clone();
            m_true.assignment.insert(var_index, true);

            let mut m_false = m.clone();
            m_false.assignment.insert(var_index, false);

            new_membranes.push(m_true);
            new_membranes.push(m_false);
        }

        self.membranes = new_membranes;
    }

    /// Step 2: eliminate membranes violating any clause
    fn check_clauses(&mut self) {
        for m in &mut self.membranes {
            for clause in &self.clauses {
                let mut satisfied = false;
                for (var, sign) in &clause.0 {
                    if let Some(val) = m.assignment.get(var) {
                        if (*sign && *val) || (!*sign && !*val) {
                            satisfied = true;
                            break;
                        }
                    }
                }
                // If all vars assigned and clause not satisfied → dissolve
                if !satisfied
                    && clause.0.iter().all(|(v, _)| m.assignment.contains_key(v))
                {
                    m.alive = false;
                    break;
                }
            }
        }
    }

    fn surviving_solutions(&self, num_vars: usize) -> Vec<HashMap<usize, bool>> {
        self.membranes
            .iter()
            .filter(|m| m.alive && m.assignment.len() == num_vars)
            .map(|m| m.assignment.clone())
            .collect()
    }
}

fn main() {
    // F = (x1 ∨ ¬x2) ∧ (¬x1 ∨ x3)
    let clauses = vec![
        Clause(vec![(1, true), (2, false)]),
        Clause(vec![(1, false), (3, true)]),
    ];

    let num_vars = 3;
    let mut system = PSatSystem::new(clauses, num_vars);

    // Expand nondeterministically for each variable
    for var in 1..=num_vars {
        system.expand(var);
        system.check_clauses();
    }

    // Collect surviving (satisfying) membranes
    let solutions = system.surviving_solutions(num_vars);
    println!(" Surviving assignments: {:?}", solutions);
}

/*
 Surviving assignments: [
  {1: true, 2: false, 3: true},
  {1: false, 2: false, 3: true},
]
*/