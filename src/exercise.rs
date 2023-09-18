#[derive(Debug, PartialEq)]
pub struct Exercise {
    pub sets: u64,
    pub reps: u64,
    pub actual_sets: u64,
    pub actual_reps: Vec<u64>
}

impl Exercise {
    pub fn new(sets: u64, reps: u64) -> Self {
        let actual_reps = Vec::with_capacity(sets as usize);
        Exercise { 
            sets,
            reps,
            actual_sets: 0, 
            actual_reps
        }
    }
}