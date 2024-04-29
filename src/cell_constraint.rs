use std::collections::HashSet;

use crate::{assignment_constraint::AssignmentConstraint, board};

pub struct CellConstraint {
    assignment_constraint: AssignmentConstraint,
    cell_column: usize,
    cell_row: usize,

}

impl CellConstraint{
    pub fn new(cell_column: usize, cell_row: usize) -> CellConstraint {
        CellConstraint {
            assignment_constraint: AssignmentConstraint::new(),
            cell_column,
            cell_row,
        }
    }    

    pub fn get_row(& self) -> usize {
        self.cell_row
    }

    pub fn get_column(& self) -> usize {
        self.cell_column
    }

    pub fn init_from_board(
        &mut self, 
        board: &board::Board, 
        constraints: &[&AssignmentConstraint],
    ) -> bool {
        if ! board.is_available(self.cell_column, self.cell_row) {
            return true;
        }

        let mut iter = constraints.iter();

        if let Some(first_constraint) = iter.next() {
            self.assignment_constraint.assign(first_constraint);
            for other_constraint in iter {
                self.assignment_constraint.intersect_with(other_constraint)
            }
        }
        self.assignment_constraint.get_allowed_values().len() > 0

    }

    pub fn has_single_possible_assignment(& self) -> bool {
        self.assignment_constraint.get_allowed_values().len() == 1
    }

    pub fn has_no_possible_assignments(& self) -> bool {
        self.assignment_constraint.get_allowed_values().is_empty()
    }

    pub fn get_allowed_values(& self) -> &HashSet<u32> {
        self.assignment_constraint.get_allowed_values()
    }    

    pub fn get_possible_assignment(& self) -> u32 {
        if let Some(candidate) = self.assignment_constraint.get_allowed_values().iter().next() {
            return *candidate;
        }
        return board::EMPTY_CELL_VALUE
    }

    pub fn mark_as_assigned(&mut self) {
        self.assignment_constraint.clear()
    }

    pub fn can_assign_value(& self, value: u32) -> bool {
        self.assignment_constraint.can_assign_value(value)
    }

    pub fn assign_value(&mut self, value: u32) -> bool {
        self.assignment_constraint.assign_value(value)
    }

    pub fn assign(&mut self, other: &CellConstraint) {
        self.assignment_constraint.assign(&other.assignment_constraint);
    }
}

