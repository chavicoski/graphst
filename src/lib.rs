//! # Graphst
//!
//! `graphst` is a library to create and manipulate graphs. It also provides some
//! implementations of popular graph algorithms.

mod graph;
pub use graph::Graph;

mod dgraph;
pub use dgraph::DGraph; // Directed Graph

pub mod algorithm;
