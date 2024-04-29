use std::cmp;

use crate::{board::Board, board_constraints::BoardConstraints, guesses_generation::{generate_group_of_number_guesses, generate_groups_of_cell_guesses}};

#[derive(Debug)]
pub struct SolutionStats {
    pub total_tried_guesses: u32,
    pub max_reached_depth:u32,
    pub depth_limit: u32,
}

impl SolutionStats {
    pub fn new() -> SolutionStats {
        SolutionStats {
            total_tried_guesses: 0,
            max_reached_depth: 0,
            depth_limit: 0
        }
    }
}

#[derive(Clone, Copy)]
pub enum GuessMethod{
    GuessMethodCell,
    GuessMethodNumber
}

pub struct SudokuGame {
    latest_solution_stats: SolutionStats,
}

impl SudokuGame {
    pub fn new() -> SudokuGame {
        SudokuGame {
            latest_solution_stats: SolutionStats::new()
        }
    }

    pub fn solve(&mut self, board: Board, guess_method: GuessMethod, backtracking_depth_limit: u32) -> (bool, Board) {
        self.latest_solution_stats = SolutionStats::new();
        self.latest_solution_stats.depth_limit = backtracking_depth_limit;

        let mut constraints = BoardConstraints::new(board);
        let mut status: bool = constraints.init();
    
        if status {
            status = self.search_solution(&mut constraints, guess_method, backtracking_depth_limit);
        }   
    
        let mut result = Board::new();
        result.assign(constraints.get_board());
        (status, result)
        
    }

    pub fn get_latest_solution_stats(&self) -> &SolutionStats {
        &self.latest_solution_stats
    }
     
     fn search_solution(&mut self, constraints: &mut BoardConstraints, guess_method: GuessMethod, backtracking_depth_limit: u32) -> bool {
        if constraints.has_conflicts() {
            return false;
        }
    
        constraints.improve_following_constraints();
        if constraints.get_board().is_full() {
            return true;
        } else {
            if constraints.has_conflicts() {
                return false;
            }
            else {
                return self.search_solution_with_backtracking(constraints, guess_method, backtracking_depth_limit);
            }
        }
     }
    
     fn search_solution_with_backtracking(&mut self, constraints: &mut BoardConstraints, guess_method: GuessMethod, backtracking_depth_limit: u32) -> bool {
        let mut found_good_guess = false;
    
        self.latest_solution_stats.max_reached_depth = cmp::max(self.latest_solution_stats.max_reached_depth, self.latest_solution_stats.depth_limit - backtracking_depth_limit);
        if backtracking_depth_limit <= 0 {
            return false;
        }
    
        let guesses = match guess_method {
            GuessMethod::GuessMethodCell => generate_groups_of_cell_guesses(constraints.get_cell_constraints()),
            GuessMethod::GuessMethodNumber => generate_group_of_number_guesses(constraints.get_cell_constraints())
        };
    
        for guess_group in guesses {
            for guess in guess_group.get_guesses() {
                let mut pending_constraints = BoardConstraints::new(Board::new());
                pending_constraints.assign(&constraints);
    
                pending_constraints.assign_value_to_cell(guess.column,guess.row,guess.value);
                self.latest_solution_stats.total_tried_guesses+=1;
    
                if self.search_solution(&mut pending_constraints, guess_method, backtracking_depth_limit-1) {
                    found_good_guess = true;
                    constraints.assign(&pending_constraints);
                    break;
                }
            }
            if found_good_guess {
                break;
            }
        }
    
        found_good_guess
     }
}

