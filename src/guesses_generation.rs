use std::cmp::Ordering;

use crate::{assignment::Assignment, cell_constraint::CellConstraint, guess_group::GuessGroup};

pub fn guess_group_compare(a: &GuessGroup, b: &GuessGroup) -> Ordering {
    if a.rank() != b.rank() {
        return a.rank().cmp(&b.rank());
    }

    for (a_assignment, b_assignment) in a.get_guesses().iter().zip(b.get_guesses().iter()) {
        if a_assignment.column != b_assignment.column {
            return a_assignment.column.cmp(&b_assignment.column)
        }

        if a_assignment.row != b_assignment.row {
            return a_assignment.row.cmp(&b_assignment.row)
        }

        if a_assignment.value != b_assignment.value {
            return a_assignment.value.cmp(&b_assignment.value)
        }
    }    

    Ordering::Equal
}

pub fn generate_groups_of_cell_guesses(cell_constraints:&[CellConstraint]) -> Vec<GuessGroup> {
    let mut result: Vec<GuessGroup> = Vec::new();


    for cell_constraint in cell_constraints {
        if !cell_constraint.has_possible_assignments() {
            continue;
        }

        let mut guess_group = GuessGroup::new();
        for assignment_number in cell_constraint.get_allowed_values().iter() {
            guess_group.add(Assignment {
                column:cell_constraint.get_column(),
                row: cell_constraint.get_row(),
                value: *assignment_number
            })
        }
        result.push(guess_group)
    }

    result.sort_by(guess_group_compare);

    result

}

pub fn generate_group_of_number_guesses(cell_constraints:&[CellConstraint]) -> Vec<GuessGroup> {
    let mut result: Vec<GuessGroup> = Vec::new();

    for _ in 0..9 {
        result.push(GuessGroup::new())
    }


    for cell_constraint in cell_constraints {
        if !cell_constraint.has_possible_assignments() {
            continue;
        }

        for assignment_number in cell_constraint.get_allowed_values().iter() {
            let guess_group =  result.get_mut((assignment_number-1) as usize).unwrap();

            guess_group.add(Assignment {
                column:cell_constraint.get_column(),
                row: cell_constraint.get_row(),
                value: *assignment_number
            })
        }


        result.retain(|item| item.rank() > 0)
        
    }

    result.sort_by(guess_group_compare);

    result
}