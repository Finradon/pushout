// src/io/neo4j.rs

use petgraph::graph::{Graph, NodeIndex};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

/// The per-node data extracted from Neo4j JSON.
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

/// Errors that can occur while parsing Neo4j JSON.
#[derive(Debug, Error)]
pub enum Neo4jError {
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("expected relationship array of length 3, got {0}")]
    BadRelLength(usize),
    #[error("unexpected relationship format")]
    BadRelFormat,
}

/// Parse a Neo4j‐style JSON export into a `petgraph::Graph<NodeData,String>`.
/// 
/// # Errors
/// Returns `Err(Neo4jError::Json(_))` if the input isn't valid JSON,
/// or `Err(Neo4jError::BadRelLength(_))` / `BadRelFormat` if a `rel` entry isn't as expected.
pub fn graph_from_neo4j_json(
    json_str: &str,
) -> Result<Graph<NodeData, String>, Neo4jError> {
    let entries: Vec<RawEntry> = serde_json::from_str(json_str)?;
    let mut graph = Graph::<NodeData, String>::new();
    let mut id_to_index: HashMap<String, NodeIndex> = HashMap::new();

    for entry in entries {
        // 1) Add or reuse the source node
        let src_idx = *id_to_index
            .entry(entry.node.id.clone())
            .or_insert_with(|| graph.add_node(entry.node.clone()));

        // 2) If there's a relation + target, add them
        if let (Some(rel_arr), Some(target)) = (entry.rel, entry.target) {
            if rel_arr.len() != 3 {
                return Err(Neo4jError::BadRelLength(rel_arr.len()));
            }
            // rel_arr should be [ {…}, "REL_TYPE", {…} ]
            let rel_type = rel_arr[1]
                .as_str()
                .ok_or(Neo4jError::BadRelFormat)?
                .to_string();

            let tgt_idx = *id_to_index
                .entry(target.id.clone())
                .or_insert_with(|| graph.add_node(target.clone()));

            graph.add_edge(src_idx, tgt_idx, rel_type);
        }
    }

    Ok(graph)
}
