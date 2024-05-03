use crate::{assignment::Assignment, assignment_constraint::AssignmentConstraint, board::Board};

pub trait CellConstraintsMap {
    fn get_cell_constraint(&self, column: usize, row: usize) -> &AssignmentConstraint;
}

pub trait RangeConstraint {
    fn get_cell_position_from_index(&self, cell_index: usize) -> (usize, usize);

    fn get_assignment_constraint(&self) -> &AssignmentConstraint;

    fn get_assignment_constraint_mut(&mut self) -> &mut AssignmentConstraint;

    fn find_single_number_assignments(
        &self,
        board: &Board,
        constraints: &impl CellConstraintsMap,
    ) -> Vec<Assignment> {
        let mut assignments: Vec<Assignment> = Vec::new();

        if !self.get_assignment_constraint().has_possible_assignments() {
            return assignments;
        }

        for value in self.get_assignment_constraint().get_allowed_values().iter() {
            let mut possible_assignments: Vec<Assignment> = Vec::new();

            for i in 0..9 {
                let (column, row) = self.get_cell_position_from_index(i);
                let cell = constraints.get_cell_constraint(column, row);
                if board.is_available(column, row) & cell.can_assign_value(*value) {
                    possible_assignments.push(Assignment {
                        column,
                        row,
                        value: *value,
                    })
                }
            }
            if possible_assignments.len() == 1 {
                assignments.push(possible_assignments.pop().unwrap());
            }
        }

        assignments
    }

    fn has_number_with_no_possible_assignments(
        &self,
        board: &Board,
        constraints: &impl CellConstraintsMap,
    ) -> bool {
        if !self.get_assignment_constraint().has_possible_assignments() {
            return false;
        }

        let mut possible_assignments_count = 0;

        for value in self.get_assignment_constraint().get_allowed_values().iter() {
            possible_assignments_count = 0;
            for i in 0..9 {
                let (column, row) = self.get_cell_position_from_index(i);
                let cell = constraints.get_cell_constraint(column, row);
                if board.is_available(column, row) & cell.can_assign_value(*value) {
                    possible_assignments_count += 1;
                }
            }

            if 0 == possible_assignments_count {
                break;
            }
        }

        possible_assignments_count == 0
    }
}
