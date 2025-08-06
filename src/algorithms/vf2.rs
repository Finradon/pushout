use petgraph::graph::{Graph, NodeIndex};
use petgraph::Direction;
use std::collections::HashMap;

/// Checks if mapping a pattern node `p` to a graph node `g` is feasible given the current mapping.
/// If `check_edge_labels` is true, then it also checks that the edge weights (or labels) match.
fn is_feasible<N, E>(
    mapping: &HashMap<NodeIndex, NodeIndex>,
    pattern: &Graph<N, E>,
    graph: &Graph<N, E>,
    p: NodeIndex,
    g: NodeIndex,
    check_edge_labels: bool,
) -> bool
where
    N: Eq,
    E: Eq,
{
    // Check node label equality
    if pattern[p] != graph[g] {
        return false;
    }

    // Outgoing edges
    for p_neighbor in pattern.neighbors_directed(p, Direction::Outgoing) {
        if let Some(&g_neighbor) = mapping.get(&p_neighbor) {
            if check_edge_labels {
                let pat_edge_idx = pattern.find_edge(p, p_neighbor).unwrap();
                let pat_weight = pattern.edge_weight(pat_edge_idx).expect("Missing pattern edge weight");
                if let Some(graph_edge_idx) = graph.find_edge(g, g_neighbor) {
                    let graph_weight = graph.edge_weight(graph_edge_idx).expect("Missing graph edge weight");
                    if pat_weight != graph_weight {
                        return false;
                    }
                } else {
                    return false;
                }
            } else if graph.find_edge(g, g_neighbor).is_none() {
                return false;
            }
        }
    }

    // Incoming edges
    for p_neighbor in pattern.neighbors_directed(p, Direction::Incoming) {
        if let Some(&g_neighbor) = mapping.get(&p_neighbor) {
            if check_edge_labels {
                let pat_edge_idx = pattern.find_edge(p_neighbor, p).unwrap();
                let pat_weight = pattern.edge_weight(pat_edge_idx).expect("Missing pattern edge weight");
                if let Some(graph_edge_idx) = graph.find_edge(g_neighbor, g) {
                    let graph_weight = graph.edge_weight(graph_edge_idx).expect("Missing graph edge weight");
                    if pat_weight != graph_weight {
                        return false;
                    }
                } else {
                    return false;
                }
            } else if graph.find_edge(g_neighbor, g).is_none() {
                return false;
            }
        }
    }

    true
}

/// Generates candidate pairs (pattern node, graph node) for mapping.
fn candidate_pairs<N, E>(
    pattern: &Graph<N, E>,
    graph: &Graph<N, E>,
    mapping: &HashMap<NodeIndex, NodeIndex>,
) -> Vec<(NodeIndex, NodeIndex)> {
    let unmapped_p: Vec<_> = pattern
        .node_indices()
        .filter(|p| !mapping.contains_key(p))
        .collect();
    let unmapped_g: Vec<_> = graph
        .node_indices()
        .filter(|g| !mapping.values().any(|&v| v == *g))
        .collect();

    if let Some(&p) = unmapped_p.first() {
        unmapped_g.into_iter().map(|g| (p, g)).collect()
    } else {
        Vec::new()
    }
}

/// Recursively collects all complete mappings (node index maps) from pattern to graph.
fn search_collect<N, E>(
    pattern: &Graph<N, E>,
    graph: &Graph<N, E>,
    mapping: &mut HashMap<NodeIndex, NodeIndex>,
    check_edge_labels: bool,
    results: &mut Vec<HashMap<NodeIndex, NodeIndex>>,
) where
    N: Eq,
    E: Eq + Clone,
{
    if mapping.len() == pattern.node_count() {
        results.push(mapping.clone());
        return;
    }
    for (p, g) in candidate_pairs(pattern, graph, mapping) {
        if is_feasible(mapping, pattern, graph, p, g, check_edge_labels) {
            mapping.insert(p, g);
            search_collect(pattern, graph, mapping, check_edge_labels, results);
            mapping.remove(&p);
        }
    }
}

/// Returns all node mapping solutions of subgraph isomorphisms from `pattern` to `graph`.
pub fn find_mappings<N, E>(
    pattern: &Graph<N, E>,
    graph: &Graph<N, E>,
    check_edge_labels: bool,
) -> Vec<HashMap<NodeIndex, NodeIndex>>
where
    N: Eq,
    E: Eq + Clone,
{
    let mut results = Vec::new();
    let mut mapping = HashMap::new();
    search_collect(pattern, graph, &mut mapping, check_edge_labels, &mut results);
    results
}

/// Returns true if at least one subgraph isomorphism exists.
pub fn vf2_subgraph_isomorphism<N, E>(
    pattern: &Graph<N, E>,
    graph: &Graph<N, E>,
    check_edge_labels: bool,
) -> bool
where
    N: Eq,
    E: Eq + Clone,
{
    !find_mappings(pattern, graph, check_edge_labels).is_empty()
}
