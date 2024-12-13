/// Solve a linear diophantine equation (or Bézout identity).
///
/// Find solution to:
/// ```txt
/// (1)  A * x + B * y = C
/// ```
///
/// by simplifying with GCD(A,B):
/// ```txt
/// (2)  A1 * x + B1 * y = C1
/// ```
///
///
/// There can be either 0 or an infinite number of solutions.
///
/// Returns
///  - A tuple containing:
///    - a1 = A / GCD(A,B)
///    - b1 = B / GCD(A,B)
///    - (x0, y0), specific solution
///
/// Note that all solutions are like:
/// `(x0 + b1*k, y0 - a1*k)`; for all k in Z
///
///
/// More details on [`Wikipedia`]
///
/// [`Wikipedia`]: https://en.wikipedia.org/wiki/B%C3%A9zout%27s_identity
pub fn solve_linear_diophantine_equation(a: i64, b: i64, c: i64) -> Option<(i64, i64, i64, i64)> {
    // Determine if solutions exist
    // Here, d = GCD(a, b)
    let (d, _, _): (i64, _, _) = extended_euclidean_algorithm(a, b);
    if c % d != 0 {
        // If c is not a multiple of the gcd, no solution exist.
        return None;
    }

    // Simplify the equation:
    // Dividing by 'd' create a situation where a1 & b1 are relatively prime.

    let a1: i64 = a / d;
    let b1: i64 = b / d;
    let c1: i64 = c / d;

    // Compute a specific solution (x0, y0) of the equation 'a1 * x + b1 * y = c1'
    let (e, x0, y0): (i64, i64, i64) = extended_euclidean_algorithm(a1, b1);
    if e != 1 {
        panic!("a1 & b1 are not relatively prime integers");
    }

    Some((a1, b1, x0 * c1, y0 * c1))
}

/// Implementation of the Extended Euclidean algorithm.
///
/// Return the GCD (greater common divisor) of provided numbers,
/// and the Bézout coefficients computed along the way.
///
/// Parameters:
///  - `a`, `b`  - Two integer values to run the algorithm on.
///
/// Returns:
///  - A tuple of three elements:
///    - `a` and `b`'s GCD,
///    - the related Bézout coefficients.
///
/// See the [`Wikipedia article`] for more details.
///
/// [`Wikipedia article`]: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
pub fn extended_euclidean_algorithm(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r): (i64, i64) = (a, b);
    let (mut old_s, mut s): (i64, i64) = (1, 0);
    let (mut old_t, mut t): (i64, i64) = (0, 1);

    while r != 0 {
        let quotient: i64 = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    // Bézout coefficients:     (old_s, old_t)
    // Greatest Common Divisor: old_r
    // Quotients by the gcd:    (t, s)
    (old_r, old_s, old_t)
}
