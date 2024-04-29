use rust_sudoku_quick_solver::{board,sudoku_game};

#[test]
fn sudoku_game_correctly_solves_puzzle_17_without_backtracking() {
    let board = board::Board {
        cells: [
            [1,0,4,0,0,0,0,0,0],
            [0,0,2,7,4,0,0,0,0],
            [0,0,0,5,0,0,0,0,0],
            [0,3,0,0,0,0,0,0,0],
            [7,5,0,0,0,0,0,0,0],
            [0,0,0,0,0,9,6,0,0],
            [0,4,0,0,0,6,0,0,0],
            [0,0,0,0,0,0,0,7,1],
            [0,0,0,0,0,1,0,3,0],
        ]
    };
    let expected_result_board = board::Board {
        cells: [
            [1,8,4,9,6,3,7,2,5],
            [5,6,2,7,4,8,3,1,9],
            [3,9,7,5,1,2,8,6,4],
            [2,3,9,6,5,7,1,4,8],
            [7,5,6,1,8,4,2,9,3],
            [4,1,8,2,3,9,6,5,7],
            [9,4,1,3,7,6,5,8,2],
            [6,2,3,8,9,5,4,7,1],
            [8,7,5,4,2,1,9,3,6],
        ]
    };    

    let mut sudoku_game = sudoku_game::SudokuGame::new();

    let (result_status,result_board) = sudoku_game.solve(board,sudoku_game::GuessMethod::GuessMethodCell,2);

    assert!(result_status);
    assert_eq!(result_board,expected_result_board);
    assert_eq!(sudoku_game.get_latest_solution_stats().total_tried_guesses,0)

}

#[test]
fn sudoku_game_correctly_solves_puzzle_too_hard_with_backtracking() {
    let board = board::Board {
        cells: [
            [2,0,0,0,0,6,0,0,0],
            [8,0,0,1,0,0,0,0,0],
            [0,7,0,0,0,9,0,0,0],
            [0,0,0,0,0,0,0,5,0],
            [0,0,9,4,0,7,3,0,0],
            [0,3,0,0,0,0,0,0,0],
            [0,0,1,7,0,0,0,3,0],
            [0,0,5,0,0,4,0,0,2],
            [6,0,0,8,0,0,0,0,4],
        ]
    };
    let expected_result_board = board::Board {
        cells: [
            [2,1,4,5,7,6,9,8,3],
            [8,9,6,1,4,3,5,2,7],
            [5,7,3,2,8,9,4,1,6],
            [4,6,2,3,9,8,7,5,1],
            [1,5,9,4,2,7,3,6,8],
            [7,3,8,6,5,1,2,4,9],
            [9,4,1,7,6,2,8,3,5],
            [3,8,5,9,1,4,6,7,2],
            [6,2,7,8,3,5,1,9,4],
        ]
    };    

    let mut sudoku_game = sudoku_game::SudokuGame::new();

    let (result_status,result_board) = sudoku_game.solve(board,sudoku_game::GuessMethod::GuessMethodCell,2);

    assert!(result_status);
    assert_eq!(result_board,expected_result_board);
    assert!(sudoku_game.get_latest_solution_stats().total_tried_guesses > 0)

}


