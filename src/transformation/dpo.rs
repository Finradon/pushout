use petgraph::graph::Graph;
use crate::algorithms::vf2::find_mappings;
use crate::transformation::{Rule, Morphism};
use crate::transformation::util::{check_gluing, delete_part, add_part};

/// Find all matches of the rule's LHS in the host graph.
/// Validates the rule before matching.
pub fn find_matches<N, E>(
    rule: &Rule<N, E>,
    host: &Graph<N, E>,
    check_edge_labels: bool,
) -> Vec<Morphism>
where
    N: Eq + Clone,
    E: Eq + Clone,
{
    // Ensure the DPO rule is well-formed
    assert!(rule.validate(), "DPO rule validation failed");
    // Use VF2-based matcher to get node mappings
    let node_maps = find_mappings(&rule.lhs, host, check_edge_labels);
    node_maps
        .into_iter()
        .map(|node_map| {
            let mut m = Morphism::new();
            for (p_node, h_node) in node_map {
                m.insert_node(p_node, h_node);
            }
            m
        })
        .collect()
}

/// Apply a single DPO rewrite at the given match morphism.
/// Returns the rewritten graph or `None` if gluing fails.
pub fn apply_once<N, E>(
    rule: &Rule<N, E>,
    host: &Graph<N, E>,
    m: &Morphism,
) -> Option<Graph<N, E>>
where
    N: Clone,
    E: Clone,
{
    // Validate rule consistency
    assert!(rule.validate(), "DPO rule validation failed");

    let mut result = host.clone();
    // Check gluing condition before rewriting
    if !check_gluing(&result, m, rule) {
        return None;
    }
    // Perform delete and add steps
    delete_part(&mut result, m, rule);
    add_part(&mut result, m, rule);
    Some(result)
}

/// Apply the rule exhaustively until no more matches exist.
/// Returns all endpoint graphs where the rule can no longer apply.
pub fn apply<N, E>(
    rule: &Rule<N, E>,
    host: &Graph<N, E>,
    check_edge_labels: bool,
) -> Vec<Graph<N, E>>
where
    N: Eq + Clone,
    E: Eq + Clone,
{
    // Validate rule once up front
    assert!(rule.validate(), "DPO rule validation failed");

    fn recurse<N, E>(
        rule: &Rule<N, E>,
        current: Graph<N, E>,
        check_edge_labels: bool,
        results: &mut Vec<Graph<N, E>>,
    )
    where
        N: Eq + Clone,
        E: Eq + Clone,
    {
        let matches = find_matches(rule, &current, check_edge_labels);
        if matches.is_empty() {
            results.push(current);
        } else {
            for m in matches {
                if let Some(next) = apply_once(rule, &current, &m) {
                    recurse(rule, next, check_edge_labels, results);
                }
            }
        }
    }

    let mut finals = Vec::new();
    recurse(rule, host.clone(), check_edge_labels, &mut finals);
    finals
}
