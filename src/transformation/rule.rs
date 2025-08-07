use petgraph::graph::Graph;
use crate::transformation::morphism::Morphism;
use std::fmt::Display;
// use std::collections::HashMap;

/// A DPO rewrite rule defined by a span of morphisms L <- K -> R.
///
/// - `lhs`: left-hand side graph L.
/// - `interface`: interface graph K (common part).
/// - `rhs`: right-hand side graph R.
/// - `l2k`: morphism from L to K.
/// - `k2r`: morphism from K to R.
pub struct Rule<N, E> {
    pub lhs: Graph<N, E>,
    pub interface: Graph<N, E>,
    pub rhs: Graph<N, E>,
    pub l2k: Morphism,
    pub k2r: Morphism,
}

impl<N, E> Rule<N, E>
where
    N: Clone,
    E: Clone,
{
    /// Construct a new DPO rule from its components.
    pub fn new(
        lhs: Graph<N, E>,
        interface: Graph<N, E>,
        rhs: Graph<N, E>,
        l2k: Morphism,
        k2r: Morphism,
    ) -> Self {
        Rule { lhs, interface, rhs, l2k, k2r }
    }

    /// Validate that the morphisms are well-defined between the graphs.
    /// Checks that:
    /// 1. Nodes in `lhs` map to nodes in `interface`.
    /// 2. Edges in `lhs` map to edges in `interface` with matching endpoints.
    /// 3. Nodes in `interface` map to nodes in `rhs`.
    /// 4. Edges in `interface` map to edges in `rhs` with matching endpoints.
    pub fn validate(&self) -> bool {
        // 1. Validate l2k node mappings: source in lhs, target in interface
        for (&l_node, &k_node) in &self.l2k.node_map {
            if !self.lhs.node_indices().any(|n| n == l_node) {
                return false;
            }
            if !self.interface.node_indices().any(|n| n == k_node) {
                return false;
            }
        }
        // 2. Validate l2k edge mappings: domain edges map and endpoints preserved
        for (&l_edge, &k_edge) in &self.l2k.edge_map {
            if let (Some((l_src, l_dst)), Some((k_src, k_dst))) = (
                self.lhs.edge_endpoints(l_edge),
                self.interface.edge_endpoints(k_edge)
            ) {
                let mapped_src = self.l2k.node_map.get(&l_src).unwrap();
                let mapped_dst = self.l2k.node_map.get(&l_dst).unwrap();
                if *mapped_src != k_src || *mapped_dst != k_dst {
                    return false;
                }
            } else {
                return false;
            }
        }
        // 3. Validate k2r node mappings: source in interface, target in rhs
        for (&k_node, &r_node) in &self.k2r.node_map {
            if !self.interface.node_indices().any(|n| n == k_node) {
                return false;
            }
            if !self.rhs.node_indices().any(|n| n == r_node) {
                return false;
            }
        }
        // 4. Validate k2r edge mappings: domain edges map and endpoints preserved
        for (&k_edge, &r_edge) in &self.k2r.edge_map {
            if let (Some((k_src, k_dst)), Some((r_src, r_dst))) = (
                self.interface.edge_endpoints(k_edge),
                self.rhs.edge_endpoints(r_edge)
            ) {
                let mapped_src = self.k2r.node_map.get(&k_src).unwrap();
                let mapped_dst = self.k2r.node_map.get(&k_dst).unwrap();
                if *mapped_src != r_src || *mapped_dst != r_dst {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl<N, E> Rule<N, E>
where
    N: Clone + Display,
    E: Clone + Display,
{
    pub fn to_cypher(&self) -> String {
        assert!(self.validate(), "DPO rule validation failed");

        // 1) LHS node variable (only one in your example)
        let lhs_node = self.lhs.node_indices().next().unwrap();
        let n0 = format!("n{}", lhs_node.index());
        let name0 = format!("{}", self.lhs.node_weight(lhs_node).unwrap());

        // 2) MATCH just that node by name
        let mut cy = format!(
            "MATCH ({n0} {{name: \"{name0}\"}})\nWITH {n0}\n",
            n0 = n0,
            name0 = name0
        );

        // 3) Determine new nodes = nodes in RHS not in interface
        let inv_k2r = self.k2r.invert();
        let mut new_vars = Vec::new();
        for node in self.rhs.node_indices() {
            if inv_k2r.map_node(&node).is_none() {
                let var = format!("r{}", node.index());
                let name = format!("{}", self.rhs.node_weight(node).unwrap());
                new_vars.push((var, name));
            }
        }

        // 4) Emit one MERGE per new node
        for (var, name) in &new_vars {
            cy.push_str(&format!("MERGE ({var} {{name: \"{name}\"}})\n", var=var, name=name));
        }

        // 5) Emit CREATE for each new relationship in RHS\Interface
        let mut rel_lines = Vec::new();
        for edge in self.rhs.edge_indices() {
            if inv_k2r.map_edge(&edge).is_none() {
                let (u, v) = self.rhs.edge_endpoints(edge).unwrap();
                let uvar = if let Some(&_k) = inv_k2r.map_node(&u) {
                    // interface node: reuse n0
                    n0.clone()
                } else {
                    format!("r{}", u.index())
                };
                let vvar = if let Some(&_k) = inv_k2r.map_node(&v) {
                    n0.clone()
                } else {
                    format!("r{}", v.index())
                };
                let label = self.rhs.edge_weight(edge).unwrap();
                rel_lines.push(format!("({})-[:{}]->({})", uvar, label, vvar));
            }
        }
        if !rel_lines.is_empty() {
            cy.push_str("CREATE ");
            cy.push_str(&rel_lines.join(", "));
            cy.push('\n');
        }

        cy
    }
}

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Write the given rule’s Cypher export to the specified file path.
///  
/// # Errors
/// Returns any I/O error encountered while creating or writing the file.
pub fn write_cypher_to_file<N, E>(
    rule: &Rule<N, E>,
    path: &Path,
) -> std::io::Result<()>
where
    N: Clone + Display,
    E: Clone + Display,
{
    let cy = rule.to_cypher();
    let mut f = File::create(path)?;
    f.write_all(cy.as_bytes())
}
