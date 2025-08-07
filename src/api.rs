// src/api.rs

use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;

use crate::algorithms::vf2::{find_mappings as vf2_find_mappings, vf2_subgraph_isomorphism};
use crate::transformation::{
    find_matches as dpo_find_matches, apply_once, apply, Rule, Morphism,
};
use crate::io::neo4j::{graph_from_neo4j_json, NodeData, Neo4jError};
use crate::transformation::rule::write_cypher_to_file;
use std::fmt::Display;
use std::path::Path;
/// Options for VF2‐based subgraph matching.
#[derive(Debug, Clone)]
pub struct MatchOptions {
    /// Whether to return *all* matches (true) or just the first (false).
    pub find_all: bool,
    /// Whether to require edge‐label equality (true) or ignore labels (false).
    pub check_edge_labels: bool,
}

impl Default for MatchOptions {
    fn default() -> Self {
        Self { find_all: true, check_edge_labels: true }
    }
}

/// Run VF2 subgraph matching of `pattern` in `host`.
/// Returns a vec of node‐to‐node maps (pattern→host).
pub fn match_subgraphs<N, E>(
    pattern: &Graph<N, E>,
    host: &Graph<N, E>,
    opts: MatchOptions,
) -> Vec<HashMap<NodeIndex, NodeIndex>>
where
    N: Eq + Clone,
    E: Eq + Clone,
{
    let all_maps = vf2_find_mappings(pattern, host, opts.check_edge_labels);
    if opts.find_all {
        all_maps
    } else {
        all_maps.into_iter().take(1).collect()
    }
}

/// Quick check: does `pattern` appear in `host`?
pub fn has_subgraph<N, E>(
    pattern: &Graph<N, E>,
    host: &Graph<N, E>,
    check_edge_labels: bool,
) -> bool
where
    N: Eq + Clone,
    E: Eq + Clone,
{
    vf2_subgraph_isomorphism(pattern, host, check_edge_labels)
}

/// A trait for “apply‐once” graph‐rewrites.
pub trait GraphRewrite<N, E> {
    /// Try to apply this rewrite to `host`.
    fn apply(&self, host: &Graph<N, E>) -> Option<Graph<N, E>>;
}

impl<N, E> GraphRewrite<N, E> for Rule<N, E>
where
    N: Eq + Clone,
    E: Eq + Clone,
{
    fn apply(&self, host: &Graph<N, E>) -> Option<Graph<N, E>> {
        if let Some(m) = dpo_find_matches(self, host, true).into_iter().next() {
            apply_once(self, host, &m)
        } else {
            None
        }
    }
}

/// Apply a single DPO rule once (returns `None` if no match or gluing fails).
pub fn apply_rule<N, E>(
    host: &Graph<N, E>,
    rule: &Rule<N, E>,
) -> Option<Graph<N, E>>
where
    N: Eq + Clone,
    E: Eq + Clone,
{
    rule.apply(host)
}

/// Sequentially apply a list of rules, threading the graph forward.
/// Each rule is applied at most once (if it matches).
pub fn apply_rules<N, E>(
    host: &Graph<N, E>,
    rules: &[Rule<N, E>],
) -> Graph<N, E>
where
    N: Eq + Clone,
    E: Eq + Clone,
{
    let mut current = host.clone();
    for rule in rules {
        if let Some(next) = apply_rule(&current, rule) {
            current = next;
        }
    }
    current
}

/// Exhaustively apply a single rule (all rewrite branches).
/// Returns **all** resulting graphs where no further match exists.
pub fn apply_exhaustive<N, E>(
    host: &Graph<N, E>,
    rule: &Rule<N, E>,
) -> Vec<Graph<N, E>>
where
    N: Eq + Clone,
    E: Eq + Clone,
{
    apply(rule, host, true)
}

/// A builder for `Rule<N,E>` to avoid manually wiring up morphisms.
pub struct RuleBuilder<N, E> {
    lhs: Option<Graph<N, E>>,
    interface: Option<Graph<N, E>>,
    rhs: Option<Graph<N, E>>,
    l2k: Option<Morphism>,
    k2r: Option<Morphism>,
}

impl<N: Clone, E: Clone> RuleBuilder<N, E> {
    /// Start a new empty builder.
    pub fn new() -> Self {
        Self {
            lhs: None,
            interface: None,
            rhs: None,
            l2k: None,
            k2r: None,
        }
    }

    pub fn lhs(mut self, g: Graph<N, E>) -> Self {
        self.lhs = Some(g);
        self
    }

    pub fn interface(mut self, g: Graph<N, E>) -> Self {
        self.interface = Some(g);
        self
    }

    pub fn rhs(mut self, g: Graph<N, E>) -> Self {
        self.rhs = Some(g);
        self
    }

    pub fn l2k(mut self, m: Morphism) -> Self {
        self.l2k = Some(m);
        self
    }

    pub fn k2r(mut self, m: Morphism) -> Self {
        self.k2r = Some(m);
        self
    }

    /// Finalize into a `Rule`. Panics if any component is missing.
    pub fn build(self) -> Rule<N, E> {
        Rule::new(
            self.lhs.expect("LHS graph required"),
            self.interface.expect("Interface graph required"),
            self.rhs.expect("RHS graph required"),
            self.l2k.expect("l2k morphism required"),
            self.k2r.expect("k2r morphism required"),
        )
    }
}



/// Parse a Neo4j-style JSON string into a `petgraph::Graph<NodeData,String>`.
/// 
/// # Errors
/// Propagates any `Neo4jError` from the underlying parser.
pub fn load_neo4j_graph(json: &str) -> Result<Graph<NodeData, String>, Neo4jError> {
    graph_from_neo4j_json(json)
}

/// Write the given DPO `Rule` as a Cypher file at `path`.
///
/// # Errors
/// Returns any I/O error encountered while creating or writing the file.
pub fn save_rule_as_cypher<N, E>(
    rule: &crate::transformation::Rule<N, E>,
    path: &Path,
) -> std::io::Result<()>
where
    N: Clone + Display,
    E: Clone + Display,
{
    write_cypher_to_file(rule, path)
}

pub fn export_rule_to_cypher<N, E>(rule: &Rule<N, E>) -> String
where
    N: Clone + Display,
    E: Clone + Display,
{
    rule.to_cypher()
}