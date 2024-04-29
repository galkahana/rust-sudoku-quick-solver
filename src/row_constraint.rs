use crate::assignment::Assignment;
use crate::assignment_constraint::AssignmentConstraint;
use crate::board::Board;
use crate::range_assignment_constraint::{CellConstraintsMap, RangeAssignmentConstraint, RangeConstraintHelper};

pub struct RowConstraint {
    range_assignment_constraint: RangeAssignmentConstraint,
    row_index: usize,
}

impl RowConstraint {
    pub fn new(row_index: usize) -> RowConstraint {
        RowConstraint {
            range_assignment_constraint: RangeAssignmentConstraint::new(),
            row_index,
        }
    }    

    pub fn init(&mut self, board: &Board) -> bool {
        let mut ok = true;
        self.range_assignment_constraint.fill_values_with_range();
        for i in 0..9 {
            if !board.is_available(i, self.row_index) {
                ok &= self.range_assignment_constraint.assign_value(board.get(i, self.row_index));
            }
        }

        ok
    }

    pub fn get_range_assignment_constraint(&self) -> &RangeAssignmentConstraint {
        &self.range_assignment_constraint
    }

    pub fn get_assignment_constraint(&self) -> &AssignmentConstraint {
        &self.range_assignment_constraint.get_assignment_constraint()
    }

    pub fn assign_value(&mut self, value: u32)-> bool {
        self.range_assignment_constraint.assign_value(value)
    }

    pub fn find_single_number_assignments(
        &self, 
        board: &Board, 
        constraints: &impl CellConstraintsMap
    ) -> Vec<Assignment> {
        self.range_assignment_constraint.find_single_number_assignments(self, board, constraints)
    }    

    pub fn assign(&mut self, other: &RowConstraint) {
        self.range_assignment_constraint.assign(&other.range_assignment_constraint);
        self.row_index = other.row_index;
    }
}

impl RangeConstraintHelper for RowConstraint {
    fn get_cell_position_from_index(
        &self,
        cell_index: usize
    ) -> (usize, usize) {
        (cell_index, self.row_index)
    }
}


