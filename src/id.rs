pub trait IDGenerator {
    type ID;

    fn get_id(&mut self) -> usize;
}

#[derive(Debug, Default)] // No copy or clone to ensure that the counter can't be duplicated
pub struct SequentialGenerator {
    counter: usize,
}

impl SequentialGenerator {
    pub fn new() -> Self {
        Self {
            counter: 0
        }
    }
}

impl IDGenerator for SequentialGenerator {
    type ID = usize;

    fn get_id(&mut self) -> usize {
        let id = self.counter;
        self.counter += 1;
        id
    }
}
