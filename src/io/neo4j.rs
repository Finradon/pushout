// src/io/neo4j.rs

use petgraph::graph::{Graph, NodeIndex};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone, Debug, Deserialize)]
pub struct NodeData {
    pub name: String,
    pub text: String,
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RawEntry {
    node: NodeData,
    rel: Option<Vec<Value>>,
    target: Option<NodeData>,
}

pub fn graph_from_neo4j_json(
    json_str: &str,
) -> Result<Graph<NodeData, String>, Box<dyn Error>> {
    let entries: Vec<RawEntry> = serde_json::from_str(json_str)?;
    let mut graph = Graph::<NodeData, String>::new();
    let mut id_to_index: HashMap<String, NodeIndex> = HashMap::new();

    for entry in entries {
        // 1) Add or reuse the source node
        let src_idx = {
            let nd = &entry.node;
            if let Some(&idx) = id_to_index.get(&nd.id) {
                idx
            } else {
                let idx = graph.add_node(nd.clone());
                id_to_index.insert(nd.id.clone(), idx);
                idx
            }
        };

        // 2) If there's a relation + target, add them
        if let (Some(rel_arr), Some(target)) = (entry.rel, entry.target) {
            if rel_arr.len() == 3 {
                // Extract the relationship type
                if let Some(rel_type) = rel_arr[1].as_str() {
                    // Add or reuse the target node
                    let tgt_idx = {
                        if let Some(&idx) = id_to_index.get(&target.id) {
                            idx
                        } else {
                            let idx = graph.add_node(target.clone());
                            id_to_index.insert(target.id.clone(), idx);
                            idx
                        }
                    };
                    // Finally add the edge
                    graph.add_edge(src_idx, tgt_idx, rel_type.to_string());
                }
            }
        }
    }

    Ok(graph)
}
