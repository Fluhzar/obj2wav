//! # Prelude
//!
//! Contains re-exports of commonly used API elements.

/// The underlying mathematical type used within higher-level types throughout this crate.
pub type Scalar = f64;

/// The common vector type used in this crate.
pub type Vector = nalgebra::Vector3<Scalar>;

pub use crate::geometry::Vertex;
pub use crate::geometry::Edge;

pub type AdjList = std::collections::BTreeMap<Vertex, Vec<Edge>>;
