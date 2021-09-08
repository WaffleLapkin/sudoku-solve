use bitvec::{array::BitArray, order::Lsb0};

use crate::{board::Board, restriction::Restriction, Digit, MaybeDigit::*, Solution, Sudoku};

pub fn solve<R>(sudoku: &Sudoku<R>) -> Result<Solution, ()>
where
    R: Restriction,
{
    let mut state = SolveState::new(sudoku);

    solve_step(&mut state)?;

    let solution = state.board.map(|x| match x {
        Definitely(n) => n,
        Hole => unreachable!("Board must be solved since `solve_step` returned `Ok(_)`"),
    });

    Ok(Solution(solution))
}

struct SolveState<'a, R> {
    holes: BitArray<Lsb0, [u8; 11]>,
    board: Board,
    restrictions: &'a R,
}

impl<'a, R> SolveState<'a, R> {
    fn new(sudoku: &'a Sudoku<R>) -> Self {
        let mut holes = BitArray::default();

        for idx in sudoku.board.cells().enumerate().flat_map(|(i, d)| match d {
            Definitely(_) => None,
            Hole => Some(i),
        }) {
            holes.set(idx, true);
        }

        Self {
            holes,
            board: sudoku.board.clone(),
            restrictions: &sudoku.restrictions,
        }
    }
}

fn solve_step<R>(state: &mut SolveState<R>) -> Result<(), ()>
where
    R: Restriction,
{
    let (x, y, idx) = match state.holes.iter_ones().next() {
        Some(hole) => (hole / 9, hole % 9, hole),
        None => return Ok(()),
    };

    state.holes.set(idx, false);

    let mut default_posibilities = BitArray::zeroed();
    default_posibilities[1..=9].set_all(true);
    for digit in state
        .restrictions
        .posibilities(x, y, &state.board, default_posibilities)
        .iter_ones()
        .map(|p| p as u8)
        .map(Digit::new)
    {
        state.board[(x, y)] = Definitely(digit);

        if let ok @ Ok(_) = solve_step(state) {
            return ok;
        }
    }

    state.board[(x, y)] = Hole;
    state.holes.set(idx, true);

    Err(())
}

#[cfg(test)]
mod tests {
    use crate::{MaybeDigit, Solution, Sudoku};

    fn test_solution(sudoku: &str, solution: &str) {
        let sudoku = Sudoku::parse(sudoku);
        let expected = Solution(Sudoku::parse(solution).board.map(|d| match d {
            MaybeDigit::Definitely(d) => d,
            _ => unreachable!(),
        }));

        let solution = super::solve(&sudoku).unwrap();
        assert_eq!(expected, solution);
    }

    #[test]
    fn basic() {
        test_solution(
            "3 7 5 6 2 _ _ _ 4
4 9 2 3 _ 8 _ 5 6
_ _ _ _ 9 4 _ 7 _
7 4 8 1 5 _ 6 3 2
1 2 _ 8 6 _ _ 4 5
6 5 9 2 _ 3 7 _ _
2 _ 4 7 3 6 5 8 9
_ 8 6 4 1 5 3 2 _
_ 3 7 _ 8 2 _ 6 1",
            "3 7 5 6 2 1 8 9 4
4 9 2 3 7 8 1 5 6
8 6 1 5 9 4 2 7 3
7 4 8 1 5 9 6 3 2
1 2 3 8 6 7 9 4 5
6 5 9 2 4 3 7 1 8
2 1 4 7 3 6 5 8 9
9 8 6 4 1 5 3 2 7
5 3 7 9 8 2 4 6 1",
        );
    }

    #[test]
    fn basic_extreme() {
        test_solution(
            "_ _ 1 _ _ _ 7 _ _
_ _ _ _ 1 _ 4 8 5
8 4 _ 6 _ _ _ 3 _
5 _ _ 1 9 _ _ _ _
_ _ 3 5 _ _ _ _ 6
_ _ _ _ _ _ 5 _ _
_ 5 9 3 _ _ 6 4 _
1 8 _ 7 2 _ _ _ 3
3 _ _ _ _ _ _ _ _",
            "2 3 1 4 5 8 7 6 9
6 9 7 2 1 3 4 8 5
8 4 5 6 7 9 2 3 1
5 6 8 1 9 7 3 2 4
9 7 3 5 4 2 8 1 6
4 1 2 8 3 6 5 9 7
7 5 9 3 8 1 6 4 2
1 8 6 7 2 4 9 5 3
3 2 4 9 6 5 1 7 8",
        );
    }

    // https://www.youtube.com/watch?v=Ui1hrp7rovw
    #[test]
    fn basic_steering_wheel() {
        test_solution(
            "_ _ _ 1 _ 2 _ _ _
_ 6 _ _ _ _ _ 7 _
_ _ 8 _ _ _ 9 _ _
4 _ _ _ _ _ _ _ 3
_ 5 _ _ _ 7 _ _ _
2 _ _ _ 8 _ _ _ 1
_ _ 9 _ _ _ 8 _ 5
_ 7 _ _ _ _ _ 6 _
_ _ _ 3 _ 4 _ _ _",
            "9 3 4 1 7 2 6 5 8
5 6 1 9 4 8 3 7 2
7 2 8 6 3 5 9 1 4
4 1 7 2 6 9 5 8 3
8 5 3 4 1 7 2 9 6
2 9 6 5 8 3 7 4 1
1 4 9 7 2 6 8 3 5
3 7 2 8 5 1 4 6 9
6 8 5 3 9 4 1 2 7",
        );
    }
}
