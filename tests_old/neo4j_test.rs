use matchbox::io::{graph_from_neo4j_json, NodeData};
// use petgraph::graph::Graph;

const SAMPLE_JSON: &str = r#"
[
  {
    "Node": {
      "name": "Bridge",
      "text": "A bridge shall exist ",
      "id": "1c1d2b9b-6080-11f0-be76-a02942b134d2"
    },
    "Rel": [
      {
        "name": "Bridge",
        "text": "A bridge shall exist ",
        "id": "1c1d2b9b-6080-11f0-be76-a02942b134d2"
      },
      "REFINE",
      {
        "name": "drainage",
        "text": "The bridge shall drain water",
        "id": "06a35402-60b2-11f0-92f6-00155d16f382"
      }
    ],
    "Target": {
      "name": "drainage",
      "text": "The bridge shall drain water",
      "id": "06a35402-60b2-11f0-92f6-00155d16f382"
    }
  },
  {
    "Node": {
      "name": "Bridge Integrity",
      "text": "The bridge shall be structurally sound.",
      "id": "2df21037-6080-11f0-b6c7-a02942b134d2"
    },
    "Rel": [
      {
        "name": "Bridge Integrity",
        "text": "The bridge shall be structurally sound.",
        "id": "2df21037-6080-11f0-b6c7-a02942b134d2"
      },
      "REFINE",
      {
        "name": "Bridge",
        "text": "A bridge shall exist ",
        "id": "1c1d2b9b-6080-11f0-be76-a02942b134d2"
      }
    ],
    "Target": {
      "name": "Bridge",
      "text": "A bridge shall exist ",
      "id": "1c1d2b9b-6080-11f0-be76-a02942b134d2"
    }
  },
  {
    "Node": {
      "name": "Substructure Integrity",
      "text": "The substructure shall be structurally sound.",
      "id": "50b5fba5-6080-11f0-b127-a02942b134d2"
    },
    "Rel": [
      {
        "name": "Substructure Integrity",
        "text": "The substructure shall be structurally sound.",
        "id": "50b5fba5-6080-11f0-b127-a02942b134d2"
      },
      "REFINE",
      {
        "name": "Bridge Integrity",
        "text": "The bridge shall be structurally sound.",
        "id": "2df21037-6080-11f0-b6c7-a02942b134d2"
      }
    ],
    "Target": {
      "name": "Bridge Integrity",
      "text": "The bridge shall be structurally sound.",
      "id": "2df21037-6080-11f0-b6c7-a02942b134d2"
    }
  },
  {
    "Node": {
      "name": "Superstructure Integrity",
      "text": "The superstructure shall be structurally sound.",
      "id": "64da78f7-6080-11f0-b6a4-a02942b134d2"
    },
    "Rel": [
      {
        "name": "Superstructure Integrity",
        "text": "The superstructure shall be structurally sound.",
        "id": "64da78f7-6080-11f0-b6a4-a02942b134d2"
      },
      "REFINE",
      {
        "name": "Bridge Integrity",
        "text": "The bridge shall be structurally sound.",
        "id": "2df21037-6080-11f0-b6c7-a02942b134d2"
      }
    ],
    "Target": {
      "name": "Bridge Integrity",
      "text": "The bridge shall be structurally sound.",
      "id": "2df21037-6080-11f0-b6c7-a02942b134d2"
    }
  },
  {
    "Node": {
      "name": "Modular Concrete",
      "text": "The bridge shall be constructed using prefabricated concrete modules",
      "id": "78608940-6080-11f0-a1a1-a02942b134d2"
    },
    "Rel": [
      {
        "name": "Modular Concrete",
        "text": "The bridge shall be constructed using prefabricated concrete modules",
        "id": "78608940-6080-11f0-a1a1-a02942b134d2"
      },
      "REFINE",
      {
        "name": "Bridge",
        "text": "A bridge shall exist ",
        "id": "1c1d2b9b-6080-11f0-be76-a02942b134d2"
      }
    ],
    "Target": {
      "name": "Bridge",
      "text": "A bridge shall exist ",
      "id": "1c1d2b9b-6080-11f0-be76-a02942b134d2"
    }
  },
  {
    "Node": {
      "name": "drainage",
      "text": "The bridge shall drain water",
      "id": "06a35402-60b2-11f0-92f6-00155d16f382"
    },
    "Rel": null,
    "Target": null
  }
]
"#;

#[test]
fn test_graph_from_neo4j_json() {
    // Parse and build
    let graph = graph_from_neo4j_json(SAMPLE_JSON).expect("failed to parse json");

    // There are 6 unique nodes:
    assert_eq!(graph.node_count(), 6);
    // And 5 REFINE relationships:
    assert_eq!(graph.edge_count(), 5);

    // Lookup by id: Bridge -> drainage edge exists
    let mut bridge_idx = None;
    let mut drainage_idx = None;
    for idx in graph.node_indices() {
        let nd: &NodeData = &graph[idx];
        if nd.id == "1c1d2b9b-6080-11f0-be76-a02942b134d2" {
            bridge_idx = Some(idx);
        }
        if nd.id == "06a35402-60b2-11f0-92f6-00155d16f382" {
            drainage_idx = Some(idx);
        }
    }
    let u = bridge_idx.expect("Bridge node missing");
    let v = drainage_idx.expect("drainage node missing");
    assert!(graph.find_edge(u, v).is_some(), "Bridge→drainage edge missing");

    // Check properties round‐tripped
    let node_data = &graph[u];
    assert_eq!(node_data.name, "Bridge");
    assert!(node_data.text.contains("A bridge shall exist"));

    // Also verify one more refine: Substructure → Bridge Integrity
    let mut sub_idx = None;
    let mut integ_idx = None;
    for idx in graph.node_indices() {
        let nd = &graph[idx];
        if nd.name == "Substructure Integrity" {
            sub_idx = Some(idx);
        }
        if nd.name == "Bridge Integrity" {
            integ_idx = Some(idx);
        }
    }
    let s = sub_idx.unwrap();
    let b = integ_idx.unwrap();
    assert!(graph.find_edge(s, b).is_some(), 
        "Substructure→Bridge Integrity refine edge missing");
}
