use crate::{
    prelude::*,
    id::{IDGenerator}
};

use std::{
    cmp::{Ordering},
};

use nalgebra::{
    Point2,
    Point3,
};

/// Struct that represents an edge between two vertices.
#[derive(Debug, Copy, Clone)]
pub struct Edge{
    vertices: (Vertex, Vertex),
    id: usize,
}

impl Edge {
    /// Creates new `Edge`.
    pub fn new(vertices: (Vertex, Vertex), gen: &mut dyn IDGenerator<ID=usize>) -> Self {
        Self {
            vertices,
            id: gen.get_id(),
        }
    }

    pub fn vertices(&self) -> (&Vertex, &Vertex) {
        (&self.vertices.0, &self.vertices.1)
    }

    /// Returns the 3D length of the edge.
    pub fn length(&self) -> f64 {
        nalgebra::distance(&Point3::from(*self.vertices.0.value()), &Point3::from(*self.vertices.1.value()))
    }

    /// Returns the 2D length of the edge.
    #[allow(non_snake_case)]
    pub fn length2D(&self) -> f64 {
        nalgebra::distance(&Point2::from(self.vertices.0.value().xy()), &Point2::from(self.vertices.1.value().xy()))
    }

    pub fn other_vertex(&self, vertex: Vertex) -> Option<Vertex> {
        if self.vertices.0 == vertex {
            Some(self.vertices.1)
        } else if self.vertices.1 == vertex {
            Some(self.vertices.0)
        } else {
            None
        }
    }

    /// Checks if the edge contains the given vertex.
    pub fn contains_vertex(&self, vertex: Vertex) -> bool {
        self.vertices.0.value_cmp(&vertex) == Some(Ordering::Equal) ||
        self.vertices.1.value_cmp(&vertex) == Some(Ordering::Equal)
    }

    /// Checks if the given edges share a vertex in common with each other.
    pub fn shares_vertex(&self, other: &Self) -> bool {
        self.contains_vertex(other.vertices.0) || self.contains_vertex(other.vertices.1)
    }

    /// Returns a vertex that is shared between the given edges, or `None` if there is no common vertex.
    pub fn shared_vertex(&self, other: &Self) -> Option<Vertex> {
        if self.shares_vertex(other) {
            if self.vertices.0.value() == other.vertices.0.value() ||
               self.vertices.0.value() == other.vertices.1.value() {
                Some(self.vertices.0)
            } else {
                Some(self.vertices.1)
            }
        } else {
            None
        }
    }
}

impl PartialEq<Edge> for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.id == other.id
    }
}

impl Eq for Edge {}

impl PartialOrd<Edge> for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
