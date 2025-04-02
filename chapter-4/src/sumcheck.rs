use ark_bn254::Fr as ScalarField;
use ark_ff::Field;
use ark_poly::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::univariate::SparsePolynomial as UnivariateSparsePolynomial;
use ark_poly::{DenseMVPolynomial, Polynomial};
use ark_std::cfg_into_iter;
use rand::Rng;

pub type MultiPoly = SparsePolynomial<ScalarField, SparseTerm>;
pub type UniPoly = UnivariateSparsePolynomial<ScalarField>;

#[derive(Clone)]
pub struct Prover {
    pub g: MultiPoly,
    pub r_vec: Vec<ScalarField>,
}

pub struct Verifier {
    pub p: Prover,
    pub c_1: ScalarField,
}

impl Prover {
    pub fn new(g: &MultiPoly) -> Self {
        Prover {
            g: g.clone(),
            r_vec: vec![],
        }
    }

    pub fn gen_uni_poly(&mut self, c: Option<ScalarField>) -> UniPoly {
        if c.is_some() {
            self.r_vec.push(c.unwrap());
        }
        let v = self.g.num_vars() - self.r_vec.len();
        (0..(2u32.pow(v as u32 - 1))).fold(
            UniPoly::from_coefficients_vec(vec![(0, 0u32.into())]),
            |sum, n| sum + self.evaluate_g_j(Prover::n_to_vec(n as usize, v)),
        )
    }

    pub fn evaluate_g_j(&self, points: Vec<ScalarField>) -> UniPoly {
        cfg_into_iter!(self.g.terms()).fold(
            UniPoly::from_coefficients_vec(vec![]),
            |sum, (coeff, term)| {
                let (coeff_eval, fixed_term) = self.evaluate_term(term, &points);
                let curr = match fixed_term {
                    None => UniPoly::from_coefficients_vec(vec![(0, coeff_eval * *coeff)]),
                    _ => UniPoly::from_coefficients_vec(vec![(
                        fixed_term.unwrap().degree(),
                        coeff_eval * *coeff,
                    )]),
                };
                curr + sum
            },
        )
    }

    pub fn slow_sum(&self) -> ScalarField {
        let v = self.g.num_vars();
        let n = 2u32.pow(v as u32);
        (0..n)
            .map(|x| self.g.evaluate(&Prover::n_to_vec(x as usize, v)))
            .sum()
    }

    fn evaluate_term(
        &self,
        term: &SparseTerm,
        points: &[ScalarField],
    ) -> (ScalarField, Option<SparseTerm>) {
        let mut fixed_term: Option<SparseTerm> = None;
        let coeff: ScalarField =
            cfg_into_iter!(term).fold(1u32.into(), |prod, (var, power)| match *var {
                j if j == self.r_vec.len() => {
                    fixed_term = Some(SparseTerm::new(vec![(j, *power)]));
                    prod
                }
                j if j < self.r_vec.len() => prod * self.r_vec[j].pow([*power as u64]),
                _ => points[*var - self.r_vec.len()].pow([*power as u64]) * prod,
            });

        (coeff, fixed_term)
    }

    fn n_to_vec(i: usize, n: usize) -> Vec<ScalarField> {
        format!("{:0>width$}", format!("{:b}", i), width = n)
            .chars()
            .map(|x| if x == '1' { 1.into() } else { 0.into() })
            .collect()
    }
}

impl Verifier {
    pub fn new(p: &Prover, c_1: ScalarField) -> Self {
        Verifier { p: p.clone(), c_1 }
    }

    fn max_degree(&self) -> Vec<usize> {
        let g = self.p.g.clone();
        let mut lookup: Vec<usize> = vec![0; g.num_vars()];
        cfg_into_iter!(g.terms.iter()).for_each(|(_, term)| {
            cfg_into_iter!(term).for_each(|(var, power)| {
                if *power > lookup[*var] {
                    lookup[*var] = *power;
                }
            })
        });
        lookup
    }

    fn get_r() -> Option<ScalarField> {
        let mut rng = rand::thread_rng();
        let r: ScalarField = rng.gen();
        Some(r)
    }

    pub fn verify(&self) -> bool {
        // 1st round
        let mut p = self.p.clone();
        let mut g_i = p.gen_uni_poly(None);
        let mut expected_c = g_i.evaluate(&0u32.into()) + g_i.evaluate(&1u32.into());
        assert_eq!(expected_c, self.c_1);

        let lookup_degree = self.max_degree();
        assert!(g_i.degree() <= lookup_degree[0]);

        // middle round
        for j in 1..p.g.num_vars() {
            let r = Verifier::get_r();
            expected_c = g_i.evaluate(&r.unwrap());
            g_i = p.gen_uni_poly(r);
            let new_c = g_i.evaluate(&0u32.into()) + g_i.evaluate(&1u32.into());
            assert_eq!(expected_c, new_c);
            assert!(g_i.degree() <= lookup_degree[j]);
        }

        // last round
        let r = Verifier::get_r();
        expected_c = g_i.evaluate(&r.unwrap());
        p.r_vec.push(r.unwrap());
        let new_c = p.g.evaluate(&p.r_vec);
        assert_eq!(expected_c, new_c);

        true
    }
}
