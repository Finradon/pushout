//! Matchbox: Pattern matching and graph rewriting library built on petgraph.

/// Core algorithms (e.g. subgraph isomorphism).
pub mod algorithms;

/// Graph transformation modules (DPO rewriting).
pub mod transformation;

pub mod io;

// Re-export key algorithmic functions
pub use algorithms::{find_mappings, vf2_subgraph_isomorphism};

// Re-export core transformation types and functions
pub use transformation::{Rule, Morphism, find_matches, apply_once, apply};

pub use io::neo4j::graph_from_neo4j_json;

pub mod api;
pub use api::{
    MatchOptions, match_subgraphs, has_subgraph,
    GraphRewrite, apply_rule, apply_rules, apply_exhaustive,
    RuleBuilder
};