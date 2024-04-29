use std::io;

pub const EMPTY_CELL_VALUE: u32 = 0;
const BOARD_SIZE: usize = 81;

#[derive(Debug)]
pub enum ReadError {
    ReadErrorIO(io::Error),
    ReadErrorNotEnoughData
}

pub type ReadResult<T> = std::result::Result<T, ReadError>;

#[derive(Debug, PartialEq)]
pub struct Board {
    pub cells: [[u32; 9]; 9],
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [
                [EMPTY_CELL_VALUE; 9]; 9
            ]
        }
    }

    /// reads board from input read buffer. Fits reading from 
    /// stdin or other io.
    /// will read 81 digits (0 marks empty cell) before returning
    /// with a respective board instance. by convention newlines can
    /// be used to mark rows. choose whatever separator you wish. again
    /// - looks for digits
    pub fn read(reader: &mut dyn io::BufRead) -> ReadResult<Board>{
        let mut board = Board::new();
        let mut digits_count: usize = 0;

        while digits_count < BOARD_SIZE {
            let mut buffer = String::new();

            match reader.read_line(&mut buffer) {
                Ok(0) => break,
                Ok(_) => {
                    for digit in buffer
                                        .trim()
                                        .chars()
                                        .filter_map(|x| x.to_digit(10)) {
                        if digits_count >= BOARD_SIZE {
                            break;
                        }

                        board.set(digits_count % 9, digits_count / 9, digit);
                        digits_count+=1;

                    }    
                },
                Err(err) => {
                    return Err(ReadError::ReadErrorIO(err));
                }
            }
        }

        if digits_count < BOARD_SIZE {
            return Err(ReadError::ReadErrorNotEnoughData);
        }
        
        Ok(board)
    }

    pub fn write(&self, writer: &mut dyn io::Write) -> io::Result<()> {
        for y in 0..9 {
            for x in 0..9 {
                writer.write_all(self.get(x,y).to_string().as_bytes())?;
            }
            writeln!(writer,"")?;
        }
        Ok(())
    }

    pub fn set(&mut self, x: usize, y: usize, value: u32){
        self.cells[y][x] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        self.cells[y][x]
    }

    pub fn is_available(&self, x: usize, y: usize) -> bool {
        self.get(x,y) == EMPTY_CELL_VALUE
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().flatten().all(|&element| element != EMPTY_CELL_VALUE)
    }

    pub fn assign(& mut self, other: &Self) {
        for x in 0..9 {
            for y in 0..9 {
                self.set(x,y, other.get(x,y))
            }
        }
    }

    pub fn clear(& mut self) {
        for x in 0..9 {
            for y in 0..9 {
                self.set(x,y, EMPTY_CELL_VALUE);
            }
        }
    }    


}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use super::*;

    #[test]
    fn new_board_is_all_empty() {
        let board = Board::new();

        for &row in board.cells.iter() {
            for &cell in row.iter() {
                assert_eq!(cell, EMPTY_CELL_VALUE);
            }
        }
    }

    #[test]
    fn read_board_creates_new_read_board() -> Result<(), ReadError> {
        let expected_result = Board {
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
        
        let read_source = "104000000\n002740000\n000500000\n030000000\n750000000\n000009600\n040006000\n000000071\n000001030\n";
        let mut reader = BufReader::new(Cursor::new(read_source.as_bytes()));

        assert_eq!(Board::read(&mut reader)?, expected_result);
        Ok(())
    }
}