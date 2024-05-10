use std::collections::HashSet;

use crate::board::EMPTY_CELL_VALUE;

pub struct AssignmentConstraint {
    allowed_values: HashSet<u32>,
}

impl AssignmentConstraint {
    pub fn new() -> AssignmentConstraint {
        AssignmentConstraint {
            allowed_values: HashSet::from_iter(1..=9),
        }
    }

    pub fn assign_value(&mut self, value: u32) -> bool {
        self.allowed_values.remove(&value)
    }

    pub fn can_assign_value(&self, value: u32) -> bool {
        self.allowed_values.contains(&value)
    }

    pub fn has_possible_assignments(&self) -> bool {
        self.allowed_values.len() > 0
    }

    pub fn has_one_possible_assignment(&self) -> bool {
        self.allowed_values.len() == 1
    }

    pub fn get_possible_assignment(&self) -> u32 {
        *self
            .get_allowed_values()
            .iter()
            .next()
            .unwrap_or(&EMPTY_CELL_VALUE)
    }

    pub fn clear(&mut self) {
        self.allowed_values.clear()
    }

    pub fn assign(&mut self, other: &AssignmentConstraint) {
        self.allowed_values.clone_from(&other.allowed_values)
    }

    pub fn intersect_with(&mut self, other: &AssignmentConstraint) {
        self.allowed_values
            .retain(|value| other.allowed_values.contains(value))
    }

    pub fn get_allowed_values(&self) -> &HashSet<u32> {
        &self.allowed_values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interset_only_keeps_common_values() {
        let expected_result = HashSet::from([2, 4]);

        let mut this = AssignmentConstraint {
            allowed_values: HashSet::from([1, 2, 3, 4]),
        };

        let that = AssignmentConstraint {
            allowed_values: HashSet::from([2, 4, 5, 6]),
        };

        this.intersect_with(&that);

        assert_eq!(this.allowed_values, expected_result);
    }
}
