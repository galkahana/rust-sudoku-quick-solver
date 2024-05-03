use std::convert::identity;

use crate::{
    assignment::Assignment, assignment_constraint::AssignmentConstraint,
    block_constraint::BlockConstraint, board::Board, column_constraint::ColumnConstraint,
    range_constraint::CellConstraintsMap, range_constraint::RangeConstraint,
    row_constraint::RowConstraint,
};

pub struct BoardConstraints {
    row_constraints: [RowConstraint; 9],
    col_constraints: [ColumnConstraint; 9],
    block_constraints: [[BlockConstraint; 3]; 3],
    cell_constraints: [AssignmentConstraint; 81],

    board: Board,
}

fn cell_coordinate_to_index(column: usize, row: usize) -> usize {
    row * 9 + column
}

impl BoardConstraints {
    pub fn new(board: Board) -> BoardConstraints {
        BoardConstraints {
            row_constraints: [
                RowConstraint::new(0),
                RowConstraint::new(1),
                RowConstraint::new(2),
                RowConstraint::new(3),
                RowConstraint::new(4),
                RowConstraint::new(5),
                RowConstraint::new(6),
                RowConstraint::new(7),
                RowConstraint::new(8),
            ],
            col_constraints: [
                ColumnConstraint::new(0),
                ColumnConstraint::new(1),
                ColumnConstraint::new(2),
                ColumnConstraint::new(3),
                ColumnConstraint::new(4),
                ColumnConstraint::new(5),
                ColumnConstraint::new(6),
                ColumnConstraint::new(7),
                ColumnConstraint::new(8),
            ],
            block_constraints: [
                [
                    BlockConstraint::new(0, 0),
                    BlockConstraint::new(3, 0),
                    BlockConstraint::new(6, 0),
                ],
                [
                    BlockConstraint::new(0, 3),
                    BlockConstraint::new(3, 3),
                    BlockConstraint::new(6, 3),
                ],
                [
                    BlockConstraint::new(0, 6),
                    BlockConstraint::new(3, 6),
                    BlockConstraint::new(6, 6),
                ],
            ],
            cell_constraints: [
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
                AssignmentConstraint::new(),
            ],

            board,
        }
    }

    pub fn init(&mut self) -> bool {
        if !self.setup_group_constraints() {
            return false;
        }

        if !self.setup_cell_constraints() {
            return false;
        }

        if self.has_conflicts() {
            return false;
        }

        return true;
    }

    pub fn has_conflicts(&self) -> bool {
        self.has_cell_with_no_possible_assignment() || self.has_number_with_no_possible_assignment()
    }

    pub fn improve_following_constraints(&mut self) -> bool {
        let mut improved = false;
        let mut might_have_more_to_improve = true;

        while might_have_more_to_improve {
            might_have_more_to_improve = self.improve_cells_with_single_possible_assignment()
                | self.improve_numbers_with_single_possible_assignment();

            if might_have_more_to_improve {
                improved = true
            }
        }

        improved
    }

    pub fn assign(&mut self, other_constraints: &BoardConstraints) {
        self.board.assign(other_constraints.get_board());
        for i in 0..9 {
            self.row_constraints[i].assign(&other_constraints.row_constraints[i]);
            self.col_constraints[i].assign(&other_constraints.col_constraints[i]);
            self.block_constraints[i / 3][i % 3]
                .assign(&other_constraints.block_constraints[i / 3][i % 3]);

            for j in 0..9 {
                self.cell_constraints[cell_coordinate_to_index(j, i)]
                    .assign(&other_constraints.cell_constraints[cell_coordinate_to_index(j, i)]);
            }
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_cell_constraints(&self) -> &[AssignmentConstraint] {
        &self.cell_constraints
    }

    pub fn assign_value_to_cell(&mut self, column: usize, row: usize, value: u32) -> () {
        self.board.set(column, row, value);
        self.cell_constraints[cell_coordinate_to_index(column, row)].clear();

        self.row_constraints[row]
            .get_assignment_constraint_mut()
            .assign_value(value);
        self.col_constraints[column]
            .get_assignment_constraint_mut()
            .assign_value(value);
        self.block_constraints[row / 3][column / 3]
            .get_assignment_constraint_mut()
            .assign_value(value);

        for i in 0..9 {
            let (x, y) = self.row_constraints[row].get_cell_position_from_index(i);
            self.cell_constraints[cell_coordinate_to_index(x, y)].assign_value(value);

            let (x, y) = self.col_constraints[column].get_cell_position_from_index(i);
            self.cell_constraints[cell_coordinate_to_index(x, y)].assign_value(value);

            let (x, y) =
                self.block_constraints[row / 3][column / 3].get_cell_position_from_index(i);
            self.cell_constraints[cell_coordinate_to_index(x, y)].assign_value(value);
        }
    }

    fn setup_group_constraints(&mut self) -> bool {
        let mut ok = true;

        for i in 0..9 {
            ok &= self.row_constraints[i].init(&self.board);
            ok &= self.col_constraints[i].init(&self.board);
            ok &= self.block_constraints[i / 3][i % 3].init(&self.board);
        }

        ok
    }

    fn setup_cell_constraints(&mut self) -> bool {
        let mut ok = true;

        for i in 0..9 {
            for j in 0..9 {
                ok &= self.init_cell_constraint(j, i);
            }
        }

        ok
    }

    fn init_cell_constraint(&mut self, column: usize, row: usize) -> bool {
        let assignment_constraint =
            &mut self.cell_constraints[cell_coordinate_to_index(column, row)];

        if !self.board.is_available(column, row) {
            assignment_constraint.clear();
            return true;
        }

        assignment_constraint.intersect_with(
            self.block_constraints[row / 3][column / 3].get_assignment_constraint(),
        );
        assignment_constraint
            .intersect_with(self.col_constraints[column].get_assignment_constraint());
        assignment_constraint.intersect_with(self.row_constraints[row].get_assignment_constraint());

        assignment_constraint.has_possible_assignments()
    }

    fn has_cell_with_no_possible_assignment(&self) -> bool {
        self.cell_constraints
            .iter()
            .enumerate()
            .any(|(index, element)| {
                !element.has_possible_assignments() && self.board.is_available(index % 9, index / 9)
            })
    }

    fn has_number_with_no_possible_assignment(&self) -> bool {
        self.row_constraints
            .iter()
            .any(|element| element.has_number_with_no_possible_assignments(&self.board, self))
            || self
                .col_constraints
                .iter()
                .any(|element| element.has_number_with_no_possible_assignments(&self.board, self))
            || self
                .block_constraints
                .iter()
                .flatten()
                .any(|element| element.has_number_with_no_possible_assignments(&self.board, self))
    }

    fn improve_cells_with_single_possible_assignment(&mut self) -> bool {
        let mut improved = false;

        for i in 0..9 {
            for j in 0..9 {
                if self.board.is_available(j, i)
                    && self.cell_constraints[cell_coordinate_to_index(j, i)]
                        .has_one_possible_assignment()
                {
                    improved = true;
                    self.assign_value_to_cell(
                        j,
                        i,
                        self.cell_constraints[cell_coordinate_to_index(j, i)]
                            .get_possible_assignment(),
                    )
                }
            }
        }

        improved
    }

    fn improve_numbers_with_single_possible_assignment(&mut self) -> bool {
        self.improve_rows() | self.improve_columns() | self.improve_blocks()
    }

    fn improve_from_assignments(&mut self, assignments: Vec<Assignment>) -> bool {
        for single_number_assignment in assignments.iter() {
            self.assign_value_to_cell(
                single_number_assignment.column,
                single_number_assignment.row,
                single_number_assignment.value,
            )
        }
        assignments.len() > 0
    }

    fn improve_rows(&mut self) -> bool {
        (0..9)
            .map(|i| {
                self.improve_from_assignments(
                    self.row_constraints[i].find_single_number_assignments(&self.board, self),
                )
            })
            .any(identity)
    }

    fn improve_columns(&mut self) -> bool {
        (0..9)
            .map(|i| {
                self.improve_from_assignments(
                    self.col_constraints[i].find_single_number_assignments(&self.board, self),
                )
            })
            .any(identity)
    }

    fn improve_blocks(&mut self) -> bool {
        (0..9)
            .map(|i| {
                self.improve_from_assignments(
                    self.block_constraints[i % 3][i / 3]
                        .find_single_number_assignments(&self.board, self),
                )
            })
            .any(identity)
    }
}

impl CellConstraintsMap for BoardConstraints {
    fn get_cell_constraint(&self, column: usize, row: usize) -> &AssignmentConstraint {
        &self.cell_constraints[cell_coordinate_to_index(column, row)]
    }
}
