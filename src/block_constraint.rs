use crate::assignment::Assignment;
use crate::assignment_constraint::AssignmentConstraint;
use crate::board::Board;
use crate::range_assignment_constraint::{CellConstraintsMap, RangeAssignmentConstraint, RangeConstraintHelper};

pub struct BlockConstraint {
    range_assignment_constraint: RangeAssignmentConstraint,
    left_column_index: usize,
    top_row_index: usize,
}

impl BlockConstraint {
    pub fn new(left_column_index: usize, top_row_index: usize) -> BlockConstraint {
        BlockConstraint {
            range_assignment_constraint: RangeAssignmentConstraint::new(),
            left_column_index,
            top_row_index,
        }
    }    

    pub fn init(&mut self, board: &Board) -> bool {
        let mut ok = true;

        for i in 0..9 {
            let (column_index, row_index) = self.get_cell_position_from_index(i);
            if !board.is_available(column_index, row_index) {
                ok &= self.range_assignment_constraint.assign_value(board.get(column_index, row_index));
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

    pub fn assign(&mut self, other: &BlockConstraint) {
        self.range_assignment_constraint.assign(&other.range_assignment_constraint);
        self.left_column_index = other.left_column_index;
        self.top_row_index = other.top_row_index;
    }     
}

impl RangeConstraintHelper for BlockConstraint {
    fn get_cell_position_from_index(
        &self,
        cell_index: usize
    ) -> (usize, usize) {
        (cell_index % 3 + self.left_column_index, cell_index / 3 + self.top_row_index)
    }        
}


