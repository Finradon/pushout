use petgraph::graph::Graph;
use matchbox::algorithms::vf2::find_mappings;

#[test]
fn test_vf2_found() {
    let mut graph = Graph::<&str, &str>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    graph.add_edge(a, b, "ab");
    graph.add_edge(b, c, "bc");
    graph.add_edge(c, d, "cd");
    graph.add_edge(a, d, "ad");

    let mut pattern = Graph::<&str, &str>::new();
    let p1 = pattern.add_node("A");
    let p2 = pattern.add_node("B");
    let p3 = pattern.add_node("C");
    pattern.add_edge(p1, p2, "ab");
    pattern.add_edge(p2, p3, "bc");

    let mappings = find_mappings(&pattern, &graph, true);
    assert!(!mappings.is_empty());
}

#[test]
fn test_vf2_not_found() {
    let mut graph = Graph::<&str, &str>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.add_edge(a, b, "ab");
    graph.add_edge(b, c, "bc");

    let mut pattern = Graph::<&str, &str>::new();
    let p1 = pattern.add_node("A");
    let p2 = pattern.add_node("B");
    let p3 = pattern.add_node("D");
    pattern.add_edge(p1, p2, "ab");
    pattern.add_edge(p2, p3, "bd");

    let mappings = find_mappings(&pattern, &graph, true);
    assert!(mappings.is_empty());
}