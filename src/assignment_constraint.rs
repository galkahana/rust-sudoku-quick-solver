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
    fn new_creates_assignment_constraint_with_all_values() {
        let constraint = AssignmentConstraint::new();
        let expected_values: HashSet<u32> = (1..=9).collect();

        assert_eq!(*constraint.get_allowed_values(), expected_values);
    }

    #[test]
    fn assign_value_removes_value_from_allowed_values() {
        let mut constraint = AssignmentConstraint::new();
        let value = 5;

        assert!(constraint.can_assign_value(value));
        assert!(constraint.assign_value(value));
        assert!(!constraint.can_assign_value(value));
    }

    #[test]
    fn can_assign_value_returns_true_if_value_is_allowed() {
        let constraint = AssignmentConstraint::new();
        let value = 3;

        assert!(constraint.can_assign_value(value));
    }

    #[test]
    fn can_assign_value_returns_false_if_value_is_not_allowed() {
        let constraint = AssignmentConstraint::new();
        let value = 10;

        assert!(!constraint.can_assign_value(value));
    }

    #[test]
    fn has_possible_assignments_returns_true_if_there_are_allowed_values() {
        let constraint = AssignmentConstraint::new();

        assert!(constraint.has_possible_assignments());
    }

    #[test]
    fn has_possible_assignments_returns_false_if_there_are_no_allowed_values() {
        let mut constraint = AssignmentConstraint::new();
        constraint.clear();

        assert!(!constraint.has_possible_assignments());
    }

    #[test]
    fn has_one_possible_assignment_returns_true_if_there_is_only_one_allowed_value() {
        let mut constraint = AssignmentConstraint::new();
        for value in 1..=9 {
            if value != 5 {
                constraint.assign_value(value);
            }
        }

        assert!(constraint.has_one_possible_assignment());
    }

    #[test]
    fn has_one_possible_assignment_returns_false_if_there_are_multiple_allowed_values() {
        let constraint = AssignmentConstraint::new();

        assert!(!constraint.has_one_possible_assignment());
    }

    #[test]
    fn get_possible_assignment_returns_any_allowed_value() {
        let mut constraint = AssignmentConstraint::new();
        constraint.assign_value(1);
        constraint.assign_value(7);

        let possible_assignment = constraint.get_possible_assignment();
        assert!(constraint.can_assign_value(possible_assignment));
    }

    #[test]
    fn get_possible_assignment_returns_zero_when_no_possible_assignments() {
        let mut constraint = AssignmentConstraint::new();
        constraint.clear();

        assert_eq!(constraint.get_possible_assignment(), 0);
    }

    #[test]
    fn clear_removes_all_allowed_values() {
        let mut constraint = AssignmentConstraint::new();
        constraint.clear();

        assert_eq!(constraint.get_allowed_values().len(), 0);
    }

    #[test]
    fn assign_copies_allowed_values_from_another_constraint() {
        let mut constraint1 = AssignmentConstraint::new();
        let constraint2 = AssignmentConstraint {
            allowed_values: HashSet::from([2, 4, 6]),
        };

        constraint1.assign(&constraint2);

        assert_eq!(
            constraint1.get_allowed_values(),
            constraint2.get_allowed_values()
        );
    }

    #[test]
    fn intersect_with_retains_common_values_with_another_constraint() {
        let expected_result = HashSet::from([2, 4]);

        let mut constraint1 = AssignmentConstraint {
            allowed_values: HashSet::from([1, 2, 3, 4]),
        };

        let constraint2 = AssignmentConstraint {
            allowed_values: HashSet::from([2, 4, 5, 6]),
        };

        constraint1.intersect_with(&constraint2);

        assert_eq!(constraint1.allowed_values, expected_result);
    }
}
