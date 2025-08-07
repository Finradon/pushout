// tests/cypher_tests.rs

use matchbox::transformation::{Rule, Morphism};
use petgraph::graph::Graph;

#[test]
fn test_cypher_export_add_edge() {
    // Build a simple DPO rule that matches a single node "A" and then adds node "B" 
    // plus an edge A–[:ab]→B.
    let mut lhs = Graph::<&str, &str>::new();
    let la = lhs.add_node("A");

    let mut interface = Graph::<&str, &str>::new();
    let ka = interface.add_node("A");

    let mut rhs = Graph::<&str, &str>::new();
    let ra = rhs.add_node("A");
    let rb = rhs.add_node("B");
    rhs.add_edge(ra, rb, "ab");

    let mut l2k = Morphism::new();
    l2k.insert_node(la, ka);

    let mut k2r = Morphism::new();
    k2r.insert_node(ka, ra);

    let rule = Rule::new(lhs, interface, rhs, l2k, k2r);

    // Export to Cypher
    let cypher = rule.to_cypher();

    // It should at least include:
    //  - a MATCH (or MERGE) for the preserved node,
    //  - a CREATE for the new node/edge,
    //  - the relationship type "ab".
    assert!(
        cypher.contains("MATCH") || cypher.contains("MERGE"),
        "Cypher must match or merge preserved elements: {}",
        cypher
    );
    assert!(
        cypher.contains("CREATE"),
        "Cypher must create new parts: {}",
        cypher
    );
    assert!(
        cypher.contains("ab"),
        "Cypher must include the relationship type: {}",
        cypher
    );
}
