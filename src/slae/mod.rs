use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

use rand::{Rng, RngCore};
use rand::seq::SliceRandom;

use crate::base::runner::TryFromVerbal;
use crate::base::tests_supply::{Test, TestsNode, TestsSupplier};
use crate::slae::Answer::FOUND;
use std::ops::Range;

type MathResult = Result<(), ()>;

#[derive(Clone, PartialEq)]
pub struct Matrix {
    n: i32,
    rows: Vec<i32>,
    matrix: Vec<f32>,
}

impl Matrix {
    pub fn new(n: i32, matrix: Vec<f32>) -> Matrix {
        Matrix {
            n,
            rows: (0..n).collect(),
            matrix,
        }
    }

    pub fn new_empty(n: i32) -> Matrix {
        Matrix {
            n,
            rows: (0..n).collect(),
            matrix: vec![0.0; (n * (n + 1)) as usize],
        }
    }

    fn ensure_bounds(&self, x: i32, y: i32) {
        if x < 0 || x > self.n || y < 0 || y >= self.n {
            panic!("Out of bounds")
        }
    }

    fn idx(&self, y: i32, x: i32) -> usize {
        return (self.rows.get(y as usize).unwrap() * (self.n + 1) + x) as usize;
    }

    pub fn get_at(&self, y: i32, x: i32) -> i32 {
        self.ensure_bounds(x, y);

        return self.matrix[self.idx(y, x)] as i32;
    }

    pub fn set_at(&mut self, y: i32, x: i32, value: i32) {
        self.ensure_bounds(x, y);

        let idx = self.idx(y, x);
        self.matrix[idx] = value as f32;
    }

    pub fn swap_rows(&mut self, a: i32, b: i32) {
        self.ensure_bounds(0, a);
        self.ensure_bounds(0, b);

        self.rows.swap(a as usize, b as usize);
    }

    pub fn sum_rows(&mut self, transmitter: i32, receiver: i32, factor: i32) -> MathResult {
        if transmitter < 0 || transmitter >= self.n {
            panic!("Out of bounds")
        }
        if receiver < 0 || receiver >= self.n {
            panic!("Out of bounds")
        }

        let mut before = Vec::new();

        let mut failed = false;
        for x in 0..self.n + 1 {
            let a = self.get_at(transmitter, x).checked_mul(factor);
            if a.is_none() {
                failed = true;
                break;
            }
            let a = a.unwrap();
            let b = self.get_at(receiver, x);
            let sum = a.checked_add(b);
            if sum.is_none() {
                failed = true;
                break;
            }
            let sum = sum.unwrap();
            if (sum as f32) as i32 != sum {
                failed = true;
                break;
            }
            before.push(b);
            self.set_at(receiver, x, sum);
        }

        if failed {
            for idx in 0..before.len() {
                self.set_at(receiver, idx as i32, *before.get(idx).unwrap())
            }
            return Err(());
        }
        Ok(())
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(&format!("{}\n", self.n));
        for i in 0..self.n {
            for j in 0..self.n + 1 {
                result.push_str(&format!("{} ", self.get_at(i, j)));
            }
            result.push('\n')
        }
        f.write_str(&result)
    }
}

#[derive(Clone)]
pub enum Answer {
    MANY,
    NONE,
    FOUND(Vec<f32>),
}

impl PartialEq<Self> for Answer {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Answer::MANY => {
                matches!(other, Answer::MANY)
            }
            Answer::NONE => {
                matches!(other, Answer::NONE)
            }
            FOUND(e) => {

                if let Answer::FOUND(me) = self {
                    if me.len() != e.len() { return false; }

                    for i in 0..me.len() {
                        if f32::abs(me.get(i).unwrap() - e.get(i).unwrap()) > 0.0001  {
                            return false;
                        }
                    }

                    true
                } else {
                    return false;
                }
            }
        }
    }
}

impl Eq for Answer {}

impl TryFromVerbal for Answer {
    fn try_from(s: &str) -> Result<Self, String> {
        if s == "no solutions" {
            return Ok(Answer::NONE);
        }
        if s == "many solutions" {
            return Ok(Answer::MANY);
        }


        let answer = s
            .split_whitespace()
            .into_iter()
            .map(|a| { f32::from_str(a).map_err(|a| a.to_string()) })
            .collect::<Result<Vec<f32>, String>>()?;

        Ok(FOUND(answer))
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FOUND(v) => {
                for x in v {
                    f.write_str(&x.to_string())?;
                    f.write_char(' ')?;
                }
            }
            Self::MANY => {
                f.write_str("many solutions")?;
            }
            Self::NONE => {
                f.write_str("no solutions")?;
            }
        }
        Ok(())
    }
}

pub struct SLAESupplier {}

impl SLAESupplier {
    pub fn new() -> SLAESupplier {
        SLAESupplier {}
    }
}

impl TestsSupplier<Matrix, Answer> for SLAESupplier {
    fn supply_tests(&self, rng: &mut dyn RngCore) -> TestsNode<Matrix, Answer> {
        let mut result = TestsNode::new("root");

        self.hand_made(result.child("hand-made"));
        self.random_many_answer(
            result.child("random many answer long"),
            rng,
            40,
            5..6,
            -100..100,
            1..30,
            4,
            10
        );
        self.random_one_answer(
            result.child("random one answer simple"),
            rng,
            200,
            1..5+1,
            -10..10,
            1..2,
            2,
            2
        );
        self.random_one_answer(
            result.child("random one answer long"),
            rng,
            40,
            5..10,
            -100..100,
            1..30,
            10,
            10
        );

        self.random_no_answer(
            result.child("random no answer short"),
            rng,
            200,
            2..5+1,
            -10..10,
            1..2,
            2,
            2
        );
        self.random_no_answer(
            result.child("random no answer long"),
            rng,
            40,
            5..10,
            -100..100,
            1..30,
            10,
            10
        );
        self.random_many_answer(
            result.child("random many answer short"),
            rng,
            200,
            2..5+1,
            -10..10,
            1..2,
            2,
            2
        );
        result
    }
}

fn shuffle_matrix(matrix: &mut Matrix, rng: &mut dyn RngCore, max_factor: i32, max_sums: i32) {
    let sums = rng.gen_range(0..max_sums);
    for _ in 0..sums {
        let a = rng.gen_range(0..matrix.n);
        let b = rng.gen_range(0..matrix.n);
        let factor = rng.gen_range(0..max_factor);

        matrix.sum_rows(a, b, factor);
    }

    if rng.gen_bool(0.9) {
        matrix.rows.shuffle(rng)
    }
}

impl SLAESupplier {

    fn nice_floatizible(&self, rng: &mut dyn RngCore, range: Range<i32>) -> i32 {
        let mut ans = rng.gen_range(range.clone());
        while ((ans as f32) as i32)!= ans  {
            ans = rng.gen_range(range.clone());
        }

        return ans;
    }

    fn non_zero(&self, rng: &mut dyn RngCore, range: Range<i32>) -> i32 {
        let mut ans = 0;
        while ans == 0 {
            ans = self.nice_floatizible(rng, range.clone());
        }
        return ans;
    }

    fn hand_made(&self, node: &mut TestsNode<Matrix, Answer>) {
        node.add_test(Test::new(
            Matrix::new(3,
                        vec![
                            1.0, 2.0, 3.0, 4.0,
                            5.0, 6.0, 7.0, 8.0,
                            9.0, 10.0, 11.0, 12.0,
                        ],
            ),
            Answer::MANY,
        ));
    }
    fn random_many_answer(&self,
                        node: &mut TestsNode<Matrix, Answer>,
                        rng: &mut dyn RngCore,
                        num: usize,
                        size: Range<i32>,
                        answer_range: Range<i32>,
                        divider: Range<i32>,
                        max_factor: i32,
                        max_sums: i32

    ) {
        for i in 0..num {
            let mut matrix = Matrix::new_empty(rng.gen_range(size.clone()));
            for i in 0..matrix.n-1 {
                matrix.set_at(i, i, self.non_zero(rng, divider.clone()));
            }
            for i in 0..matrix.n-1 {
                matrix.set_at(i, matrix.n, self.non_zero(rng, answer_range.clone()));
            }
            if i == 32 {
                println!("{}", matrix);
            }


            let sums = matrix.n * max_sums;
            println!("{}", matrix);
            shuffle_matrix(&mut matrix, rng, max_factor, sums);
            println!("{}", matrix);

            node.add_test(Test::new(matrix, Answer::MANY));
        }
    }
    fn random_no_answer(&self,
                         node: &mut TestsNode<Matrix, Answer>,
                         rng: &mut dyn RngCore,
                         num: usize,
                         size: Range<i32>,
                         answer_range: Range<i32>,
                         divider: Range<i32>,
                         max_factor: i32,
                         max_sums: i32

    ) {
        for _ in 0..num {
            let mut matrix = Matrix::new_empty(rng.gen_range(size.clone()));
            for i in 0..matrix.n-1 {
                matrix.set_at(i, i, self.nice_floatizible(rng, divider.clone()));
            }
            for i in 0..matrix.n-1 {
                matrix.set_at(i, matrix.n, self.nice_floatizible(rng, answer_range.clone()));
            }


            let mut ans = 0;
            while ans == 0 {
                ans = self.nice_floatizible(rng, answer_range.clone());
            }
            matrix.set_at(matrix.n - 1, matrix.n, ans);

            let sums = matrix.n * max_sums;
            shuffle_matrix(&mut matrix, rng, max_factor, sums);

            node.add_test(Test::new(matrix, Answer::NONE));
        }
    }
    fn random_one_answer(&self,
                         node: &mut TestsNode<Matrix, Answer>,
                         rng: &mut dyn RngCore,
        num: usize,
        size: Range<i32>,
        answer_range: Range<i32>,
        divider: Range<i32>,
        max_factor: i32,
        max_sums: i32

    ) {
        for _ in 0..num {
            let mut matrix = Matrix::new_empty(rng.gen_range(size.clone()));
            let mut answer = Vec::new();
            for i in 0..matrix.n {
                matrix.set_at(i, i, self.nice_floatizible(rng, divider.clone()));
            }
            for i in 0..matrix.n {
                matrix.set_at(i, matrix.n, self.nice_floatizible(rng, answer_range.clone()));

                answer.push((matrix.get_at(i, matrix.n) as f32) / (matrix.get_at(i, i) as f32));
            }

            let sums = matrix.n * max_sums;
            shuffle_matrix(&mut matrix, rng, max_factor, sums);

            node.add_test(Test::new(matrix, Answer::FOUND(answer)));
        }
    }
}
