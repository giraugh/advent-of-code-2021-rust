#[derive(Debug, Default)]
pub struct Cell {
    number: usize,
    marked: bool,
}

impl From<usize> for Cell {
    fn from(number: usize) -> Self {
        Self {
            number,
            marked: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct Board {
    rows: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(rows: Vec<Vec<Cell>>) -> Self {
        Self { rows }
    }

    pub fn mark(&mut self, number: usize) {
        for (row, col) in self.positions() {
            if self.rows[row][col].number == number {
                self.rows[row][col].marked = true;
            }
        }
    }

    pub fn size(&self) -> usize {
        self.rows.len()
    }

    pub fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        iproduct!(0..self.size(), 0..self.size())
    }

    // Note: how to get this to work?
    // pub fn all_cells(&mut self) -> impl Iterator<Item = &mut Cell> {
    //     self.positions().map(|(i, j)| &mut self.rows[i][j])
    // }

    pub fn has_won(&self) -> bool {
        (0..self.size()).any(|i| {
            let row: usize = (0..self.size())
                .map(|j| self.rows[i][j].marked as usize)
                .sum();
            let col: usize = (0..self.size())
                .map(|j| self.rows[j][i].marked as usize)
                .sum();
            row == self.size() || col == self.size()
        })
    }

    pub fn calculate_score(&self, last_called_number: usize) -> usize {
        let unmarked_sum = self
            .positions()
            .map(|(i, j)| &self.rows[i][j])
            .filter(|c| !c.marked)
            .map(|c| c.number)
            .sum::<usize>();
        unmarked_sum * last_called_number
    }
}

impl From<&[String]> for Board {
    fn from(lines: &[String]) -> Self {
        let rows: Vec<Vec<Cell>> = lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<usize>().expect("a number").into())
                    .collect()
            })
            .collect();
        Board::new(rows)
    }
}

impl From<Vec<Vec<usize>>> for Board {
    fn from(rows: Vec<Vec<usize>>) -> Self {
        let rows = rows
            .iter()
            .map(|row| row.iter().map(|&num| num.into()).collect())
            .collect();
        Board::new(rows)
    }
}

#[test]
fn mark() {
    let mut board: Board = (vec![vec![1, 2], vec![3, 4]]).into();
    board.mark(3);
    assert!(board.rows[1][0].marked);
}

#[test]
fn has_won_row() {
    let mut board: Board = (vec![vec![1, 2], vec![3, 4]]).into();
    assert!(!board.has_won());
    board.mark(3);
    assert!(!board.has_won());
    board.mark(4);
    assert!(board.has_won());
}

#[test]
fn has_won_col() {
    let mut board: Board = (vec![vec![1, 2], vec![3, 4]]).into();
    assert!(!board.has_won());
    board.mark(1);
    assert!(!board.has_won());
    board.mark(2);
    assert!(board.has_won());
}
