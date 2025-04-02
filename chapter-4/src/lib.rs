pub mod gkr;
pub mod sumcheck;

#[cfg(test)]
mod tests {
    use crate::sumcheck::{MultiPoly, Prover, Verifier};
    use ark_poly::multivariate::{SparseTerm, Term};
    use ark_poly::DenseMVPolynomial;

    #[test]
    fn test_sumcheck() {
        let g = MultiPoly::from_coefficients_vec(
            3,
            vec![
                (2u32.into(), SparseTerm::new(vec![(0, 3)])),
                (1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
                (1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
            ],
        );

        let prover = Prover::new(&g);
        let manual_sum = prover.slow_sum();

        let verifier = Verifier::new(&prover, manual_sum);
        verifier.verify();
    }
}
