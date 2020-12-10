use crate::prelude::*;

use nalgebra::{
    Point2,
    Point3,
};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vertex(nalgebra::Point3<Scalar>);

impl Vertex {
    pub fn raw(&self) -> &nalgebra::Point3<Scalar> {
        &self.0
    }
}

impl From<nalgebra::Point3<Scalar>> for Vertex {
    fn from(p: nalgebra::Point3<Scalar>) -> Self {
        Self(p)
    }
}

impl From<Vertex> for nalgebra::Point3<Scalar> {
    fn from(v: Vertex) -> Self {
        v.0
    }
}

/// Struct that represents an edge between two vertices.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Edge(pub Vertex, pub Vertex);

impl Edge {
    /// Creates new `Edge`.
    pub fn new(verts: (Vertex, Vertex)) -> Self {
        Self (
            verts.0,
            verts.1
        )
    }

    /// Returns the 3D length of the edge.
    pub fn length(&self) -> f64 {
        nalgebra::distance(&Point3::from(self.0), &Point3::from(self.1))
    }

    /// Returns the 2D length of the edge.
    #[allow(non_snake_case)]
    pub fn length2D(&self) -> f64 {
        nalgebra::distance(&Point2::from(self.0.raw().xy()), &Point2::from(self.1.raw().xy()))
    }

    /// Checks if the edge contains the given vertex.
    pub fn contains_vertex(&self, vertex: Vertex) -> bool {
        self.0 == vertex || self.1 == vertex
    }

    /// Checks if the given edges share a vertex in common with each other.
    pub fn shares_vertex(&self, other: &Self) -> bool {
        self.contains_vertex(other.0) || self.contains_vertex(other.1)
    }

    /// Returns a vertex that is shared between the give edges, or `None` if there is no common vertex.
    pub fn shared_vertex(&self, other: &Self) -> Option<Vertex> {
        if self.shares_vertex(other) {
            if self.0 == other.0 || self.0 == other.1 {
                Some(self.0)
            } else {
                Some(self.1)
            }
        } else {
            None
        }
    }
}

impl From<(Vertex, Vertex)> for Edge {
    fn from(v: (Vertex, Vertex)) -> Self {
        Edge::new(v)
    }
}

impl From<Edge> for (Vertex, Vertex) {
    fn from(e: Edge) -> Self {
        (e.0, e.1)
    }
}
