use crate::assignment::Assignment;
use crate::assignment_constraint::AssignmentConstraint;
use crate::board::Board;
use crate::range_assignment_constraint::{CellConstraintsMap, RangeAssignmentConstraint, RangeConstraintHelper};

pub struct ColumnConstraint {
    pub range_assignment_constraint: RangeAssignmentConstraint,
    column_index: usize,
}

impl ColumnConstraint {
    pub fn new(column_index: usize) -> ColumnConstraint {
        ColumnConstraint {
            range_assignment_constraint: RangeAssignmentConstraint::new(),
            column_index,
        }
    }    

    pub fn init(&mut self, board: &Board) -> bool {
        let mut ok = true;
        self.range_assignment_constraint.fill_values_with_range();
        for i in 0..9 {
            if !board.is_available(self.column_index, i) {
                ok &= self.range_assignment_constraint.assign_value(board.get(self.column_index,i));
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

    pub fn assign(&mut self, other: &ColumnConstraint) {
        self.range_assignment_constraint.assign(&other.range_assignment_constraint);
        self.column_index = other.column_index;
    }
}

impl RangeConstraintHelper for ColumnConstraint {
    fn get_cell_position_from_index(
        &self,
        cell_index: usize
    ) -> (usize, usize) {
        (self.column_index, cell_index)
    }
}
