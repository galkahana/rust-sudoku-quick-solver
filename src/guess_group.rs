use crate::assignment::Assignment;

pub struct GuessGroup {
    guesses: Vec<Assignment>,
}

impl GuessGroup {
    pub fn new() -> GuessGroup {
        GuessGroup {
            guesses: Vec::new(),
        }
    }

    pub fn get_guesses(&self) -> &Vec<Assignment> {
        &self.guesses
    }

    pub fn rank(&self) -> usize {
        self.guesses.len()
    }

    pub fn add(&mut self, assignment: Assignment) {
        self.guesses.push(assignment);
    }
}