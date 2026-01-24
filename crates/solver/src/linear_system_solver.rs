#[derive(Clone, Debug)]
struct Fraction {
    num: i64,
    den: i64,
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

impl Fraction {
    fn new(num: i64, den: i64) -> Self {
        assert!(den != 0);
        let mut num = num;
        let mut den = den;
        if den < 0 {
            num = -num;
            den = -den;
        }
        let g = gcd(num, den);
        Fraction {
            num: num / g,
            den: den / g,
        }
    }

    fn zero() -> Self {
        Fraction { num: 0, den: 1 }
    }

    fn is_zero(&self) -> bool {
        self.num == 0
    }

    #[allow(dead_code)]
    fn add(&self, other: &Fraction) -> Fraction {
        Fraction::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }

    fn sub(&self, other: &Fraction) -> Fraction {
        Fraction::new(
            self.num * other.den - other.num * self.den,
            self.den * other.den,
        )
    }

    fn mul(&self, other: &Fraction) -> Fraction {
        Fraction::new(self.num * other.num, self.den * other.den)
    }

    fn div(&self, other: &Fraction) -> Fraction {
        Fraction::new(self.num * other.den, self.den * other.num)
    }
}

#[allow(clippy::needless_range_loop)]
pub fn solve_smallest_nonnegative_integer(
    a: Vec<Vec<i64>>, // m x n
    b: Vec<i64>,      // m
) -> Option<Vec<i64>> {
    let m = a.len();
    let n = a[0].len();

    // Augmented matrix
    let mut mat = vec![vec![Fraction::zero(); n + 1]; m];
    for i in 0..m {
        for j in 0..n {
            mat[i][j] = Fraction::new(a[i][j], 1);
        }
        mat[i][n] = Fraction::new(b[i], 1);
    }

    let mut row = 0;
    let mut pivot_col = vec![None; n];

    // Gaussian elimination
    for col in 0..n {
        // find pivot
        let mut pivot = None;
        for r in row..m {
            if !mat[r][col].is_zero() {
                pivot = Some(r);
                break;
            }
        }
        if let Some(p) = pivot {
            mat.swap(row, p);

            let inv = mat[row][col].clone();
            for c in col..=n {
                mat[row][c] = mat[row][c].div(&inv);
            }

            for r in 0..m {
                if r != row && !mat[r][col].is_zero() {
                    let factor = mat[r][col].clone();
                    for c in col..=n {
                        mat[r][c] = mat[r][c].sub(&factor.mul(&mat[row][c]));
                    }
                }
            }

            pivot_col[col] = Some(row);
            row += 1;
        }
    }

    // Check inconsistency
    for r in 0..m {
        let mut all_zero = true;
        for c in 0..n {
            if !mat[r][c].is_zero() {
                all_zero = false;
                break;
            }
        }
        if all_zero && !mat[r][n].is_zero() {
            return None;
        }
    }

    // Smallest solution: free variables = 0
    let mut x = vec![Fraction::zero(); n];
    for col in 0..n {
        if let Some(r) = pivot_col[col] {
            x[col] = mat[r][n].clone();
        }
    }

    // Check integer & non-negative
    let mut result = vec![0i64; n];
    for i in 0..n {
        if x[i].den != 1 || x[i].num < 0 {
            return None;
        }
        result[i] = x[i].num;
    }

    Some(result)
}
