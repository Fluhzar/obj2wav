//! # Postman
//!
//! This module contains the necessary structures and function necessary to compute
//! a path across a graph, commonly referred to as the [Postman Problem][0].
//!
//! [0]: https://en.wikipedia.org/wiki/Route_inspection_problem

use crate::{
    prelude::*,
    id::IDGenerator,
};

use std::collections::BTreeMap;

use nalgebra;

pub fn path(mut vertices: Vec<Vertex>, mut map: AdjList, gen: &mut dyn IDGenerator<ID=usize>) -> Vec<Vertex> {
    map = duplicate_odd_edges(&vertices, &map, gen);

    find_path(&vertices, &map)
}

type VisitedMap = BTreeMap<Vertex, bool>;
fn graph_is_connected(skip: &Edge, verts: Vec<Vertex>, map: &AdjList) -> bool {
    let mut visit_map = VisitedMap::new();
    for v in &verts {
        visit_map.insert(*v, false);
    }

    dfs(verts[0], skip, &mut visit_map, map);

    for (_, visited) in visit_map {
        if !visited {
            return false;
        }
    }

    true
}

fn dfs(source: Vertex, skip: &Edge, visited: &mut VisitedMap, map: &AdjList) {
    visited.insert(source, true);

    if let Some(vec) = map.get(&source) {
        for v in vec {
            let neighbor = v.other_vertex(source).unwrap(); // We're iterating over `source`'s neighbors, this is safe
            if !visited.get(&neighbor).unwrap() && v != skip {
                dfs(neighbor, skip, visited, map);
            }
        }
    }
}

fn duplicate_odd_edges(vertices: &Vec<Vertex>, map: &AdjList, gen: &mut dyn IDGenerator<ID=usize>) -> AdjList {
    let mut out_map = map.clone();

    'outer: for v in vertices {
        if vertex_is_odd(v, map) {
            let mut odd_edge_found = false;
            if let Some(vec) = map.get(v) {
                'inner: for e in vec {
                    if edge_is_odd(e, map) {
                        let (a, b) = e.vertices();
                        let dup = Edge::new((a.clone(), b.clone()), gen);
                        out_map.get_mut(a).unwrap().push(dup);
                        out_map.get_mut(b).unwrap().push(dup);
                        odd_edge_found = true;
                        break 'inner;
                    }
                }

                if !odd_edge_found {
                    out_map.clear(); // Enables default problem-solving behavior of duplicating the entire graph. This is not ideal and should be changed in the future.
                    break 'outer;
                }
            }
        }
    }

    if out_map.len() == 0 {
        let mut edges: Vec<_> = map
            .values()
            .map(|v| v.iter())
            .flatten()
            .copied()
            .collect();
        let mut dups: Vec<_> = edges.iter()
            .map(|e| Edge::new(
                {
                    let (a, b) = e.vertices();
                    (*a, *b)
                },
                gen
            ))
            .collect();
        edges.append(&mut dups);

        for e in edges {
            let (a, b) = e.vertices();
            if !out_map.contains_key(a) {
                out_map.insert(*a, Vec::new());
            }
            if !out_map.contains_key(b) {
                out_map.insert(*b, Vec::new());
            }
            out_map.get_mut(a).unwrap().push(e);
            out_map.get_mut(b).unwrap().push(e);
        }
    }

    out_map
}

fn find_path(vertices: &Vec<Vertex>, map: &AdjList) -> Vec<Vertex> {
    todo!()
}

fn vertex_is_odd(vertex: &Vertex, map: &AdjList) -> bool {
    if let Some(v) = map.get(&vertex) {
        v.len() & 1 == 1
    } else {
        false
    }
}

fn edge_is_odd(edge: &Edge, map: &AdjList) -> bool {
    vertex_is_odd(edge.vertices().0, map) && vertex_is_odd(&edge.vertices().1, map)
}
