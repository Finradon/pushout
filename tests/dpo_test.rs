// tests/dpo_tests.rs

use pushout::{Rule, Morphism, find_matches, apply_once};
use petgraph::graph::Graph;

#[test]
fn test_delete_fails_on_dangling() {
    // Host: A–B–C
    let mut host = Graph::<&str, &str>::new();
    let ha = host.add_node("A");
    let hb = host.add_node("B");
    let hc = host.add_node("C");
    host.add_edge(ha, hb, "ab");
    host.add_edge(hb, hc, "bc");

    // Rule: delete B (no interface) → should fail (dangling edges)
    let mut lhs = Graph::<&str, &str>::new();
    let _lb = lhs.add_node("B");

    let interface = Graph::<&str, &str>::new(); // K empty
    let rhs = Graph::<&str, &str>::new();       // R empty
    let l2k = Morphism::new();
    let k2r = Morphism::new();
    let rule = Rule::new(lhs, interface, rhs, l2k, k2r);

    let matches = find_matches(&rule, &host, true);
    assert_eq!(matches.len(), 1, "Should find exactly one match for node B");

    let result = apply_once(&rule, &host, &matches[0]);
    assert!(result.is_none(), "apply_once should return None due to dangling edges");
}

#[test]
fn test_add_node_and_edge() {
    // Host: single node A
    let mut host = Graph::<&str, &str>::new();
    let _ha = host.add_node("A");

    // Rule: match A, then add B and an edge A->B
    let mut lhs = Graph::<&str, &str>::new();
    let la = lhs.add_node("A");

    let mut interface = Graph::<&str, &str>::new();
    let ka = interface.add_node("A");

    let mut rhs = Graph::<&str, &str>::new();
    let ra = rhs.add_node("A");
    let rb = rhs.add_node("B");
    rhs.add_edge(ra, rb, "ab");

    // Morphisms: L→K and K→R
    let mut l2k = Morphism::new();
    l2k.insert_node(la, ka);

    let mut k2r = Morphism::new();
    k2r.insert_node(ka, ra);

    let rule = Rule::new(lhs, interface, rhs, l2k, k2r);

    let matches = find_matches(&rule, &host, true);
    assert_eq!(matches.len(), 1, "Should find one match for the single node A");

    let result = apply_once(&rule, &host, &matches[0])
        .expect("apply_once should succeed for an addition rule");

    // After applying, we expect two nodes and one edge
    assert_eq!(result.node_count(), 2, "Result should have 2 nodes");
    assert_eq!(result.edge_count(), 1, "Result should have 1 edge");

    // Check that there is an edge from A→B
    let a_index = result
        .node_indices()
        .find(|&n| result[n] == "A")
        .unwrap();
    let b_index = result
        .node_indices()
        .find(|&n| result[n] == "B")
        .unwrap();
    assert!(
        result.find_edge(a_index, b_index).is_some(),
        "Expected an edge A→B to have been created"
    );
}
