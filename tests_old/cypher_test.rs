// matchbox/tests/cypher_test.rs

use matchbox::Rule;
use matchbox::transformation::Morphism;
use petgraph::graph::Graph;

fn make_delete_edge_rule() -> Rule<&'static str, &'static str> {
    // LHS: A->B
    let mut lhs = Graph::<&str, &str>::new();
    let la = lhs.add_node("A");
    let lb = lhs.add_node("B");
    lhs.add_edge(la, lb, "ab");

    // K (interface): nodes A, B, no edge
    let mut interface = Graph::<&str, &str>::new();
    let ka = interface.add_node("A");
    let kb = interface.add_node("B");

    // RHS: same nodes A, B, no edge
    let mut rhs = Graph::<&str, &str>::new();
    let ra = rhs.add_node("A");
    let rb = rhs.add_node("B");

    // Morphisms
    let mut l2k = Morphism::new();
    l2k.insert_node(la, ka);
    l2k.insert_node(lb, kb);
    let mut k2r = Morphism::new();
    k2r.insert_node(ka, ra);
    k2r.insert_node(kb, rb);

    Rule::new(lhs, interface, rhs, l2k, k2r)
}

fn make_add_edge_rule() -> Rule<&'static str, &'static str> {
    // LHS: nodes A, B, no edge
    let mut lhs = Graph::<&str, &str>::new();
    let la = lhs.add_node("A");
    let lb = lhs.add_node("B");

    // K: same nodes, no edge
    let mut interface = Graph::<&str, &str>::new();
    let ka = interface.add_node("A");
    let kb = interface.add_node("B");

    // RHS: A->B
    let mut rhs = Graph::<&str, &str>::new();
    let ra = rhs.add_node("A");
    let rb = rhs.add_node("B");
    rhs.add_edge(ra, rb, "ab");

    // Morphisms
    let mut l2k = Morphism::new();
    l2k.insert_node(la, ka);
    l2k.insert_node(lb, kb);
    let mut k2r = Morphism::new();
    k2r.insert_node(ka, ra);
    k2r.insert_node(kb, rb);

    Rule::new(lhs, interface, rhs, l2k, k2r)
}

#[test]
fn test_delete_edge_to_cypher() {
    let rule = make_delete_edge_rule();
    let cypher = rule.to_cypher();

    // It should MATCH the A->B pattern with a bound relationship variable `r0`
    assert!(cypher.contains("MATCH (n0)-[r0:ab]->(n1)"),
        "expected MATCH (n0)-[r0:ab]->(n1) in:\n{}", cypher);

    // It should DELETE the bound relationship `r0`
    assert!(cypher.contains("DELETE r0"),
        "expected DELETE r0 in:\n{}", cypher);

    // It should carry forward the interface nodes
    assert!(cypher.contains("WITH n0, n1"),
        "expected WITH n0, n1 in:\n{}", cypher);

    // CREATE should still appear (even if empty)
    assert!(cypher.contains("CREATE"),
        "expected a CREATE clause in:\n{}", cypher);
}

#[test]
fn test_add_edge_to_cypher() {
    let rule = make_add_edge_rule();
    let cypher = rule.to_cypher();

    // Even though LHS has no edges, MATCH should still be present
    assert!(cypher.starts_with("MATCH "),
        "expected MATCH at start of:\n{}", cypher);

    // DELETE clause will exist but be empty after the keyword
    assert!(cypher.contains("DELETE"),
        "expected DELETE clause in:\n{}", cypher);

    // Interface nodes should be carried
    assert!(cypher.contains("WITH n0, n1"),
        "expected WITH n0, n1 in:\n{}", cypher);

    // The new edge should be created in the CREATE clause
    assert!(cypher.contains("CREATE (n0)-[:ab]->(n1)"),
        "expected CREATE (n0)-[:ab]->(n1) in:\n{}", cypher);
}
