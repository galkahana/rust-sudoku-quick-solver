use crate::{assignment::Assignment, assignment_constraint::AssignmentConstraint, board::Board, cell_constraint::CellConstraint};

const ALL_VALUES: [u32; 9] = [1,2,3,4,5,6,7,8,9];


pub trait CellConstraintsMap {
    fn get_cell_constraint(
        & self,
        column: usize,
        row: usize,       
    ) -> & CellConstraint;
}

pub trait RangeConstraintHelper {
    fn get_cell_position_from_index(
        &self,
        cell_index: usize
    ) -> (usize, usize);
}


pub struct RangeAssignmentConstraint {
    assignment_constraint: AssignmentConstraint
}

impl RangeAssignmentConstraint{
    pub fn new() -> RangeAssignmentConstraint {
        RangeAssignmentConstraint {
            assignment_constraint: AssignmentConstraint::new(),
        }
    }

    pub fn fill_values_with_range(&mut self) {
        self.assignment_constraint.init_with_values(&ALL_VALUES);
    }

    pub fn is_full(&self) -> bool {
        self.assignment_constraint.get_allowed_values().is_empty()
    }

    pub fn find_single_number_assignments(
        &self, 
        helper: &impl RangeConstraintHelper, 
        board: &Board, 
        constraints: &impl CellConstraintsMap
    ) -> Vec<Assignment> {
        let mut assignments: Vec<Assignment> = Vec::new();

        if self.is_full() {
            return assignments
        }

        for value in  self.assignment_constraint.get_allowed_values().iter() {
            let mut possible_assignments: Vec<&CellConstraint> = Vec::new();

            for i in 0..9 {
                let (column, row) = helper.get_cell_position_from_index(i);
                let cell = constraints.get_cell_constraint(column, row);
                if board.is_available(cell.get_column(), cell.get_row()) & cell.can_assign_value(*value) {
                    possible_assignments.push(cell)    
                }
            }
            if possible_assignments.len() == 1 {
                let cell = possible_assignments.pop().unwrap();

                assignments.push(Assignment {
                    column: cell.get_column(),
                    row: cell.get_row(),
                    value: *value
                })
            }
        }

        assignments

    }

    pub fn assign_value(&mut self, value: u32) -> bool {
        self.assignment_constraint.assign_value(value)
    }

    pub fn assign(&mut self, other: &RangeAssignmentConstraint) {
        self.assignment_constraint.assign(&other.assignment_constraint);
    }

    pub fn get_assignment_constraint(&self) -> &AssignmentConstraint {
        &self.assignment_constraint
    }

    pub fn has_number_with_no_possible_assignments(
        &self, 
        helper: & impl RangeConstraintHelper, 
        board: &Board, 
        constraints: & impl CellConstraintsMap       
    ) -> bool {
        if self.is_full() {
            return false;
        }


        let mut possible_assignments_count = 0;

        for value in  self.assignment_constraint.get_allowed_values().iter() {
            possible_assignments_count= 0;
            for i in 0..9 {
                let (column, row) = helper.get_cell_position_from_index(i);
                let cell = constraints.get_cell_constraint(column, row);
                if board.is_available(cell.get_column(), cell.get_row()) & cell.can_assign_value(*value) {
                    possible_assignments_count+=1;
                }
            }

            if 0 == possible_assignments_count {
                break;
            }
        }

        possible_assignments_count == 0

    }
}
