use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use crate::transformation::rule::Rule;
use crate::transformation::morphism::Morphism;

/// Delete the image of L \ K from the host graph according to match morphism m.
/// Removes edges in L not in interface and nodes in L not in interface.
pub fn delete_part<N, E>(host: &mut Graph<N, E>, m: &Morphism, rule: &Rule<N, E>) {
    // Prepare inverted l2k mapping
    let inv_l2k = rule.l2k.invert();

    // 1. Delete edges that are in L but not in interface K
    for l_edge in rule.lhs.edge_indices() {
        if inv_l2k.map_edge(&l_edge).is_none() {
            let (l_src, l_dst) = rule.lhs.edge_endpoints(l_edge).unwrap();
            if let (Some(&h_src), Some(&h_dst)) = (m.map_node(&l_src), m.map_node(&l_dst)) {
                if let Some(e_idx) = host.find_edge(h_src, h_dst) {
                    host.remove_edge(e_idx);
                }
            }
        }
    }

    // 2. Delete nodes in L not in K (and their incident edges)
    let mut to_delete = Vec::new();
    for l_node in rule.lhs.node_indices() {
        if inv_l2k.map_node(&l_node).is_none() {
            if let Some(&h_node) = m.map_node(&l_node) {
                to_delete.push(h_node);
            }
        }
    }
    for node in to_delete {
        host.remove_node(node);
    }
}

/// Add the image of R \ K into the host graph according to match morphism m.
/// Adds nodes and edges from R not in interface, connecting via interface mapping.
pub fn add_part<N: Clone, E: Clone>(host: &mut Graph<N, E>, m: &Morphism, rule: &Rule<N, E>) {
    // Prepare inverted morphisms once
    let inv_k2r = rule.k2r.invert();
    let inv_l2k = rule.l2k.invert();

    // First, add new nodes: those in R not in interface
    let mut k_to_new: Vec<(NodeIndex, NodeIndex)> = Vec::new();
    for r_node in rule.rhs.node_indices() {
        if inv_k2r.map_node(&r_node).is_none() {
            let weight = rule.rhs.node_weight(r_node).unwrap().clone();
            let new_node = host.add_node(weight);
            k_to_new.push((r_node, new_node));
        }
    }

    // Next, add edges: those in R not in interface
    for r_edge in rule.rhs.edge_indices() {
        if inv_k2r.map_edge(&r_edge).is_none() {
            let (r_src, r_dst) = rule.rhs.edge_endpoints(r_edge).unwrap();
            // Determine host source
            let h_src = if let Some(&k_src) = inv_k2r.map_node(&r_src) {
                let via_l = inv_l2k.map_node(&k_src).unwrap();
                *m.map_node(via_l).unwrap()
            } else {
                k_to_new.iter().find(|&&(r,_)| r == r_src).unwrap().1
            };
            // Determine host target
            let h_dst = if let Some(&k_dst) = inv_k2r.map_node(&r_dst) {
                let via_l = inv_l2k.map_node(&k_dst).unwrap();
                *m.map_node(via_l).unwrap()
            } else {
                k_to_new.iter().find(|&&(r,_)| r == r_dst).unwrap().1
            };
            let e_weight = rule.rhs.edge_weight(r_edge).unwrap().clone();
            host.add_edge(h_src, h_dst, e_weight);
        }
    }
}

/// Check the gluing condition: ensure deleting L\K does not leave dangling edges in the host.
pub fn check_gluing<N, E>(host: &Graph<N, E>, m: &Morphism, rule: &Rule<N, E>) -> bool {
    for l_node in rule.lhs.node_indices() {
        if rule.l2k.map_node(&l_node).is_none() {
            if let Some(&h_node) = m.map_node(&l_node) {
                for edge in host.edges(h_node) {
                    let other = if edge.source() == h_node {
                        edge.target()
                    } else {
                        edge.source()
                    };
                    let valid = rule.lhs.node_indices().any(|l2|
                        m.map_node(&l2).map(|&n| n == other).unwrap_or(false)
                    );
                    if !valid {
                        return false;
                    }
                }
            }
        }
    }
    true
}
