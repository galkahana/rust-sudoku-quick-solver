use crate::assignment_constraint::AssignmentConstraint;
use crate::board::Board;
use crate::range_constraint::RangeConstraint;

pub struct ColumnConstraint {
    assignment_constraint: AssignmentConstraint,
    column_index: usize,
}

impl ColumnConstraint {
    pub fn new(column_index: usize) -> ColumnConstraint {
        ColumnConstraint {
            assignment_constraint: AssignmentConstraint::new(),
            column_index,
        }
    }    

    pub fn init(&mut self, board: &Board) -> bool {
        let mut ok = true;
        for i in 0..9 {
            if !board.is_available(self.column_index, i) {
                ok &= self.assignment_constraint.assign_value(board.get(self.column_index,i));
            }
        }

        ok
    }

    pub fn assign(&mut self, other: &ColumnConstraint) {
        self.assignment_constraint.assign(&other.assignment_constraint);
        self.column_index = other.column_index;
    }
}

impl RangeConstraint for ColumnConstraint {
    fn get_cell_position_from_index(
        &self,
        cell_index: usize
    ) -> (usize, usize) {
        (self.column_index, cell_index)
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


