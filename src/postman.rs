//! # Postman
//!
//! This module contains the necessary structures and function necessary to compute
//! a path across a graph, commonly referred to as the [Postman Problem][0].
//!
//! [0]: https://en.wikipedia.org/wiki/Route_inspection_problem

use std::collections::HashMap;

use nalgebra;

use crate::prelude::*;

fn duplicate_odd_edges(vertices: Vec<Vertex>, map: &HashMap<Vertex, Vec<Edge>>) -> HashMap<Vertex, Vec<Edge>> {
    todo!()
}

fn find_path(vertices: &Vec<Vertex>, map: &HashMap<Vertex, Vec<Edge>>) -> Vec<Vertex> {
    todo!()
}

fn vertex_is_odd(vertex: &Vertex, map: &HashMap<Vertex, Vec<Edge>>) -> bool {
    todo!()
}

fn edge_is_odd(edge: &Edge, map: &HashMap<Vertex, Vec<Edge>>) -> bool {
    vertex_is_odd(&edge.0, map) && vertex_is_odd(&edge.1, map)
}
