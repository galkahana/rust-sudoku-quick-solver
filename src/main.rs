
use std::fs;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use rust_sudoku_quick_solver::{board, sudoku_game};
use clap::Parser;
use std::time::Duration;
use indicatif::ProgressBar;

/// Sudoku Puzzle solver.
/// Input is a board formatted in the following manner:
/// Each cell is designated by a digit: 
///     - 1-9 for designated value
///     - 0 for empty value
/// separators between digits are not required, though
/// you can separate lines with newlines.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// input file path (defaults to stdin)
    #[arg(short, long)]
    input_file: Option<String>,

    /// output file path (defaults to stdout)
    #[arg(short, long)]
    output_file: Option<String>,
}





fn main() {
    let arguments = Args::parse();

    // read board
    let mut reader: Box<dyn BufRead> = match arguments.input_file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap()))
    };    
    let board = match  board::Board::read(&mut reader) {
        Ok(board) => board,
        Err(error) => panic!("Error while reading input {:?}", error)
    };

    let bar = ProgressBar::new_spinner();
    bar.set_message("Attempting to solve puzzle");
    bar.enable_steady_tick(Duration::from_millis(100));    

    // solve the puzzle
    let mut game = sudoku_game::SudokuGame::new();

    let (result_status, result_board) = game.solve(board, sudoku_game::GuessMethod::GuessMethodNumber, 2);

    if !result_status {
        bar.finish_with_message(format!("Could not solve puzzle. here's some stats {:?}", game.get_latest_solution_stats()));
    }
    else {
        bar.finish_with_message("Done");
    }

    // write output
    let mut writer: Box<dyn Write> = match arguments.output_file {
        None => Box::new(BufWriter::new(io::stdout())),
        Some(filename) => Box::new(BufWriter::new(fs::File::open(filename).unwrap()))
    };  


    result_board.write(&mut writer).unwrap();
}
