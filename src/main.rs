/*use serde::Serialize;
use std::collections::HashMap;
use rug::Rational;

#[derive(Serialize)]
struct Membrane {
    id: usize,
    variables: HashMap<String, f64>,
    score: f64,
}
// Mutation
use rand::Rng;

impl Membrane {
    fn mutate(&mut self, mutation_rate: f64, mutation_range: f64) {
        let mut rng = rand::thread_rng();
        for value in self.variables.values_mut() {
            if rng.gen::<f64>() < mutation_rate {
                let delta = rng.gen_range(-mutation_range..mutation_range);
                *value += delta;
            }
        }
    }
}

#[derive(Clone)]
struct Constraint {
    name: String,
    weight: f64,
    // simple linear constraint for demo: sum of variables <= limit
    limit: f64,
    vars: Vec<String>,
}

impl Constraint {
    fn satisfaction(&self, vars: &HashMap<String, f64>) -> f64 {
        let sum: f64 = self.vars.iter().map(|v| *vars.get(v).unwrap_or(&0.0)).sum();
        if sum <= self.limit {
            1.0 // fully satisfied
        } else {
            self.limit / sum // partial satisfaction
        }
    }
}

fn evaluate_membrane(m: &mut Membrane, constraints: &[Constraint]) {
    let mut total_score = 0.0;
    for c in constraints {
        total_score += c.weight * c.satisfaction(&m.variables);
    }
    m.score = total_score;
}*/

/*fn main() { // Mutation
    let constraints = vec![
        Constraint { name: "c1".to_string(), weight: 1.0, limit: 10.0, vars: vec!["x".to_string(), "y".to_string()] },
        Constraint { name: "c2".to_string(), weight: 2.0, limit: 5.0, vars: vec!["x".to_string()] },
    ];

    // Initial membranes
    let mut membranes = vec![];
    for i in 0..5 {
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), i as f64 + 1.0);
        vars.insert("y".to_string(), (5 - i) as f64);
        membranes.push(Membrane { id: i, variables: vars, score: 0.0 });
    }

    // Evolution loop
    for generation in 0..10 {
        // Evaluate scores
        for m in membranes.iter_mut() {
            evaluate_membrane(m, &constraints);
        }

        // Sort by score
        membranes.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        println!("Generation {}: Best score = {}", generation, membranes[0].score);

        // Mutation: apply to all but top 1 membrane (elitism)
        for m in membranes.iter_mut().skip(1) {
            m.mutate(0.5, 1.0); // 50% mutation rate, +/-1.0 range
        }
    }
    
    let r = Rational::from((-12, 15));
    let recip = Rational::from(r.recip_ref());
    assert_eq!(recip, Rational::from((-5,4)));
    assert_eq!(recip, -1.25);
}*/


/*fn main() -> std::io::Result<()> {
    // Example constraints
    let constraints = vec![
        Constraint { name: "c1".to_string(), weight: 1.0, limit: 10.0, vars: vec!["x".to_string(), "y".to_string()] },
        Constraint { name: "c2".to_string(), weight: 2.0, limit: 5.0, vars: vec!["x".to_string()] },
    ];

    // Create sample membranes
    let mut membranes = vec![];
    for i in 0..5 {
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), i as f64 + 1.0);
        vars.insert("y".to_string(), (5 - i) as f64);
        let mut m = Membrane { id: i, variables: vars, score: 0.0 };
        evaluate_membrane(&mut m, &constraints);
        membranes.push(m);
    }

    // Rank membranes by score
    membranes.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    // Serialize to D3.js JSON
    let json = serde_json::to_string_pretty(&membranes)?;
    std::fs::write("membranes.json", json)?;
    println!("JSON saved to membranes.json");

    Ok(())
}*/


use symbolica::{atom::{Atom, AtomCore}, parse, symbol};
use std::collections::HashMap;
use serde::Serialize;
use symbolica::evaluate::{FunctionMap, OptimizationSettings};

#[derive(Clone, Debug, Serialize)]
struct Membrane {
    id: usize,
    vars: HashMap<String, f64>,
    alive: bool,
    x: f64,
    y: f64,
}

#[derive(Clone, Debug, Serialize)]
struct SymbolicConstraint {
    expr: String,
}

#[derive(Serialize)]
struct D3Graph {
    nodes: Vec<Membrane>,
    links: Vec<Link>,
}

#[derive(Serialize)]
struct Link {
    source: usize,
    target: usize,
}

struct MembraneSystem {
    membranes: Vec<Membrane>,
    constraints: Vec<SymbolicConstraint>,
    next_id: usize,
}

impl MembraneSystem {
    fn new() -> Self {
        Self {
            membranes: vec![],
            constraints: vec![],
            next_id: 0,
        }
    }

    fn add_membrane(&mut self, vars: HashMap<String, f64>, x: f64, y: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.membranes.push(Membrane { id, vars, alive: true, x, y });
        id
    }

    fn add_constraint(&mut self, expr: &str) {
        self.constraints.push(SymbolicConstraint { expr: expr.to_string() });
    }

    fn evaluate(&mut self) {
        // Immutable function/constant environment (keep empty unless you add functions/constants)
        let fn_map = FunctionMap::new();

        for m in &mut self.membranes {
            m.alive = true;

            // Build a deterministic parameter order and corresponding values
            let mut params: Vec<Atom> = Vec::with_capacity(m.vars.len());
            let mut values: Vec<f64> = Vec::with_capacity(m.vars.len());

            let mut var_names: Vec<&str> = m.vars.keys().map(|s| s.as_str()).collect();
            var_names.sort_unstable();

            for name in var_names {
                params.push(parse!(name));          // Atom parameter
                values.push(m.vars[name]);          // f64 value
            }

            // Evaluate each constraint
            for c in &self.constraints {
                let expr: Atom = parse!(&c.expr);

                // Construct a fresh OptimizationSettings each call (no move-after-use)
                let mut evaluator = match expr
                    .evaluator(&fn_map, &params, OptimizationSettings::default())
                    .map(|ev| ev.map_coeff(&|x| x.to_real().unwrap().into()))
                {
                    Ok(ev) => ev,
                    Err(_) => { m.alive = false; break; }
                };

                let result: f64 = evaluator.evaluate_single(&values);

                if result.abs() > 1e-6 {
                    m.alive = false;
                    break;
                }
            }
        }
    }

/*fn evaluate(&mut self) { //rationalize_coefficient() error
        // One (empty) function environment is fine if you don't define custom functions.
        let fn_map = FunctionMap::new(); // supports constants/functions if you add them later
        let opt = OptimizationSettings::default();

        for m in &mut self.membranes {
            m.alive = true;

            // ---- Build a stable parameter list and corresponding value vector ----
            // IMPORTANT: keep the order of params and values aligned.
            let mut params: Vec<Atom> = Vec::with_capacity(m.vars.len());
            let mut values: Vec<f64> = Vec::with_capacity(m.vars.len());

            // We make the order deterministic by collecting and sorting the variable names.
            // (You can skip sorting if you don't care about determinism.)
            let mut var_names: Vec<&str> = m.vars.keys().map(|s| s.as_str()).collect();
            var_names.sort_unstable();

            for name in var_names {
                params.push(parse!(name));
                values.push(m.vars[name]);
            }

            // ---- Evaluate all constraints ----
            for c in &self.constraints {
                // Parse once, then ensure there are no floats in the expression tree:
                // Symbolica recommends rationalizing coefficients before numerical evaluation.
                // (This converts float literals in the expression to exact rationals.)
                let expr = parse!(&c.expr).rationalize_coefficients(); // see docs
                // Build an evaluator specialized to (fn_map, params).
                // Map exact coefficients -> f64 as the docs show.
                // You can also use `|x| x.to_real().unwrap().into()` when complex numbers might appear.
                let mut evaluator = match expr
                    .evaluator(&fn_map, &params, opt)
                    .map(|e| e.map_coeff(&|x| x.to_real().unwrap().into()))
                {
                    Ok(ev) => ev,
                    Err(_) => {
                        m.alive = false;
                        break;
                    }
                };

                // Evaluate once for this membrane.
                let result = evaluator.evaluate_single(&values);

                // Interpret result ~ 0?
                if result.abs() > 1e-6 {
                    m.alive = false;
                    break;
                }
            }
        }
    }*/

    fn export_json(&self) -> String {
        let links: Vec<Link> = self.membranes.windows(2)
            .map(|w| Link { source: w[0].id, target: w[1].id })
            .collect();

        let graph = D3Graph { nodes: self.membranes.clone(), links };
        serde_json::to_string_pretty(&graph).unwrap()
    }
}


fn main() {
    //symbolica 
    let input = parse!("x^2*log(2*x + y) + exp(3*x)");
    let a     = input.derivative(symbol!("x"));
    println!("d/dx {} = {}:", input, a);
    //
    /*
    let mut ms = MembraneSystem::new();
    ms.add_constraint("x - y");         // expect near 0 to be alive
    ms.add_constraint("x + y - 3.0");   // expect near 0 to be alive

    let mut vars = HashMap::new();
    vars.insert("x".into(), 1.0);
    vars.insert("y".into(), 2.0);
    ms.add_membrane(vars, 0.0, 0.0);

    ms.evaluate();
    assert!(ms.membranes[0].alive);
    
     */
    let mut system = MembraneSystem::new();

    let spacing = 50.0;
    for i in 0..6 {
        for j in 0..6 {
            let mut vars = HashMap::new();
            vars.insert("x".to_string(), i as f64);
            vars.insert("y".to_string(), j as f64);
            system.add_membrane(vars, i as f64 * spacing, j as f64 * spacing);
        }
    }

    // Example constraints
    system.add_constraint("x^2 + y^2 - 25");
    system.add_constraint("x");
    system.add_constraint("y");

    system.evaluate();

    let json_output = system.export_json();
    std::fs::write("membrane_graph_dynamic.json", &json_output)
        .expect("Unable to write JSON file");

    println!(" Dynamic JSON graph saved to 'membrane_graph_dynamic.json'");
}
