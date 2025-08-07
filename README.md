# Pushout

A graph rewriting library built on [petgraph](https://crates.io/crates/petgraph), providing:

* **VF2 subgraph isomorphism** for pattern matching
* **DPO (Double Pushout) algebraic graph rewriting** with rule definitions
* **Neo4j JSON import** into `petgraph::Graph`
* **Cypher export** of DPO rules for Neo4j execution

> **Crate**: [pushout](https://crates.io/crates/pushout) · **License**: Apache-2.0

---

## Features

1. **Pattern Matching** (`src/algorithms/vf2.rs`)

   * `match_subgraphs` & `has_subgraph` via the VF2 algorithm
2. **Graph Rewriting** (`src/transformation/`)

   * Define DPO rules with LHS, Interface (K), RHS graphs
   * `apply_once`, `apply_rules`, `apply_exhaustive`
   * `RuleBuilder` for ergonomic rule construction
3. **I/O**

   * `load_neo4j_graph(json: &str)` to parse Neo4j JSON exports
   * `export_rule_to_cypher` & `save_rule_as_cypher` for Cypher queries

Future work: RDF/SPARQL support via optional `graphdb` feature.

---

## Getting Started

Add to your `Cargo.toml`:

```toml
[dependencies]
pushout = "0.1.1"
petgraph = "0.8.1"
serde = { version = "1.0", features = ["derive"] }
thiserror = "2.0.12"
```

Then in your code:

```rust
use pushout::api::{
    match_subgraphs, has_subgraph,
    apply_rule, apply_rules,
    RuleBuilder,
    load_neo4j_graph, export_rule_to_cypher,
};
use petgraph::graph::Graph;

// 1. Load a Neo4j JSON export
let json = std::fs::read_to_string("graph.json")?;
let host: Graph<_, String> = load_neo4j_graph(&json)?;

// 2. Build a DPO rule
let rule = RuleBuilder::new()
    .lhs(pattern_graph)
    .interface(interface_graph)
    .rhs(replacement_graph)
    .l2k(l2k_morphism)
    .k2r(k2r_morphism)
    .build();

// 3. Apply the rule
let result = apply_rule(&host, &rule).unwrap_or(host.clone());

// 4. Export to Cypher
let cy = export_rule_to_cypher(&rule);
std::fs::write("rule.cypher", cy)?;
```

---

## License

This project is licensed under [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)

