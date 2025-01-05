#[derive(Debug, Default)]
pub struct Checker;

impl Checker {
    pub fn check_all(&self, board: &Vec<Vec<bool>>, i: usize, j: usize) -> usize {
        self.checek_hw2(board, i, j) + self.check_diagonal(board, i, j)
    }

    pub fn check_diagonal(&self, board: &Vec<Vec<bool>>, i: usize, j: usize) -> usize {
        const DIRECTION: [(i32, i32); 4] = [
            (-1, -1), // Sol üst
            (-1, 1),  // Sağ üst
            (1, -1),  // Sol alt
            (1, 1),   // Sağ alt
        ];
        let size = board.len() as i32;
        let mut conflicts: usize = 0;

        for &(row_delta, col_delta) in &DIRECTION {
            let (mut row, mut col) = (i as i32, j as i32);

            loop {
                row += row_delta;
                col += col_delta;

                if row >= 0 && row < size && col >= 0 && col < size {
                    if board[row as usize][col as usize] {
                        conflicts += 1;
                        //break; // Her yönde ilk çakışmayı bulduktan sonra dur
                    }
                } else {
                    break;
                }
            }
        }
        conflicts
    }

    pub fn checek_hw(&self, board: &Vec<Vec<bool>>, i: usize, j: usize) -> usize {
        let mut conflicts = 0;
        let size = board.len();

        for a in 0..size {
            if a == i {
                continue;
            }
            if board[a][j] {
                conflicts += 1;
            }
        }
        for b in 0..size {
            if b == j {
                continue;
            }
            if board[i][b] {
                conflicts += 1;
            }
        }
        conflicts
    }
    pub fn checek_hw2(&self, board: &Vec<Vec<bool>>, i: usize, j: usize) -> usize {
        let size = board.len();

        (0..size).filter(|&a| a != i && board[a][j]).count()
            + (0..size).filter(|&b| b != j && board[i][b]).count()
    }
}

#[derive(Debug)]
pub struct HillClimbing;

impl HillClimbing {
    pub fn find_best_all(&self, board: &mut Vec<Vec<bool>>) -> Vec<usize> {
        let mut scores: Vec<usize> = vec![];
        for a in 0..board.len() {
            let (_, j) = self.find_queen(board, a);
            scores.push(self.find_best_move(board, a, j));
        }
        scores
    }
    fn find_queen(&self, board: &Vec<Vec<bool>>, i: usize) -> (usize, usize) {
        for a in 0..board.len() {
            if board[i][a] {
                return (i, a);
            }
        }
        (0, 0)
    }
    fn find_best_move(&self, board: &mut Vec<Vec<bool>>, i: usize, j: usize) -> usize {
        board[i][j] = false;
        let (mut best_move, mut best_score) = ((0, 0), usize::MAX);
        for a in 0..board.len() {
            let res = Checker.check_all(board, i, a);
            if res < best_score {
                best_score = res;
                best_move = (i, a);
            }
        }
        board[best_move.0][best_move.1] = true;
        best_score
    }
}
