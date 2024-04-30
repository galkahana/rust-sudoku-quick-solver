use crate::assignment_constraint::AssignmentConstraint;
use crate::board::Board;
use crate::range_constraint::RangeConstraint;

pub struct BlockConstraint {
    assignment_constraint: AssignmentConstraint,
    left_column_index: usize,
    top_row_index: usize,
}

impl BlockConstraint {
    pub fn new(left_column_index: usize, top_row_index: usize) -> BlockConstraint {
        BlockConstraint {
            assignment_constraint: AssignmentConstraint::new(),
            left_column_index,
            top_row_index,
        }
    }    

    pub fn init(&mut self, board: &Board) -> bool {
        let mut ok = true;

        for i in 0..9 {
            let (column_index, row_index) = self.get_cell_position_from_index(i);
            if !board.is_available(column_index, row_index) {
                ok &= self.assignment_constraint.assign_value(board.get(column_index, row_index));
            }
        }

        ok
    }

    pub fn assign(&mut self, other: &BlockConstraint) {
        self.assignment_constraint.assign(&other.assignment_constraint);
        self.left_column_index = other.left_column_index;
        self.top_row_index = other.top_row_index;
    }     
}

impl RangeConstraint for BlockConstraint {
    fn get_cell_position_from_index(
        &self,
        cell_index: usize
    ) -> (usize, usize) {
        (cell_index % 3 + self.left_column_index, cell_index / 3 + self.top_row_index)
    }     

    fn get_assignment_constraint(
        &self
    ) -> &AssignmentConstraint {
        &self.assignment_constraint
    }

    fn get_assignment_constraint_mut(
        &mut self
    ) -> &mut AssignmentConstraint {
        &mut self.assignment_constraint
    }        
}


