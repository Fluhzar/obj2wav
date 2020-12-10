use crate::{
    prelude::*,
    id::{
        IDGenerator,
        SequentialGenerator,
    },
};

use std::cmp::Ordering;

use nalgebra::{
    Point3,
};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    value: Point3<Scalar>,
    id: usize,
}

impl Vertex {
    pub fn new<T>(value: Point3<Scalar>, gen: &mut SequentialGenerator) -> Self {
        Self {
            value,
            id: gen.get_id(),
        }
    }

    /// # Unsafe
    ///
    /// IDs are intended to be unique, so setting a custom ID is considered
    /// unsafe as only the crate-supplied generator can be guaranteed to provide
    /// a unique ID when called.
    pub(crate) unsafe fn with_id(value: Point3<Scalar>, id: usize) -> Self {
        Self {
            value,
            id
        }
    }

    pub fn mut_value(&mut self) -> &mut Point3<Scalar> {
        &mut self.value
    }

    pub fn value(&self) -> &Point3<Scalar> {
        &self.value
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn value_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value())
    }
}

impl PartialEq<Vertex> for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.id == other.id
    }
}

impl Eq for Vertex {}

impl PartialOrd<Vertex> for Vertex {
    fn partial_cmp(&self, other: &Vertex) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
