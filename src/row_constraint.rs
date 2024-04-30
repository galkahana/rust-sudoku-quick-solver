use crate::assignment_constraint::AssignmentConstraint;
use crate::board::Board;
use crate::range_constraint::RangeConstraint;

pub struct RowConstraint {
    assignment_constraint: AssignmentConstraint,
    row_index: usize,
}

impl RowConstraint {
    pub fn new(row_index: usize) -> RowConstraint {
        RowConstraint {
            assignment_constraint: AssignmentConstraint::new(),
            row_index,
        }
    }    

    pub fn init(&mut self, board: &Board) -> bool {
        let mut ok = true;
        for i in 0..9 {
            if !board.is_available(i, self.row_index) {
                ok &= self.assignment_constraint.assign_value(board.get(i, self.row_index));
            }
        }

        ok
    }

    pub fn assign(&mut self, other: &RowConstraint) {
        self.assignment_constraint.assign(&other.assignment_constraint);
        self.row_index = other.row_index;
    }
}

impl RangeConstraint for RowConstraint {
    fn get_cell_position_from_index(
        &self,
        cell_index: usize
    ) -> (usize, usize) {
        (cell_index, self.row_index)
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


