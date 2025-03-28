use ndarray::{Array1, Array2};
use rand::random;

struct X {
    field: u64,
    r: u64,
    prod: u64,
}

impl X {
    fn new(r: u64, field: u64) -> Self {
        Self { field, r, prod: 1 }
    }
}

impl Iterator for X {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.prod = (self.prod * self.r) % self.field;
        Some(self.prod)
    }
}

struct Freivalds {
    pub field: u64,
}

impl Freivalds {
    fn get_vec(&self, n: usize) -> Array1<u64> {
        let r = random::<u64>();
        let x = X::new(r % self.field, self.field);
        x.take(n).collect()
    }

    fn check_matrix_dimensions(&self, a: &Array2<u64>, b: &Array2<u64>, c: &Array2<u64>) -> bool {
        a.shape() == b.shape() && b.shape() == c.shape()
    }

    pub fn verify(&self, a: &Array2<u64>, b: &Array2<u64>, c: &Array2<u64>) -> bool {
        assert!(self.check_matrix_dimensions(a, b, c));
        let v = self.get_vec(c.ncols());
        a.dot(&b.dot(&v)) == c.dot(&v)
    }
}

pub fn freivalds_simulations() {
    let freivalds = Freivalds { field: 1009 };

    let a = Array2::from_shape_vec((2, 2), vec![1, 2, 3, 4]).unwrap();
    let b = Array2::from_shape_vec((2, 2), vec![5, 6, 7, 8]).unwrap();
    let c = Array2::from_shape_vec((2, 2), vec![19, 22, 43, 50]).unwrap();

    assert!(freivalds.verify(&a, &b, &c));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_freivalds_verify() {
        let freivalds = Freivalds { field: 1009 };

        let a = array![[1, 2], [3, 4]];
        let b = array![[5, 6], [7, 8]];
        let c = array![[19, 22], [43, 50]];

        assert!(freivalds.verify(&a, &b, &c));
    }

    #[test]
    #[should_panic]
    fn test_freivalds_verify_incorrect_dimensions() {
        let freivalds = Freivalds { field: 1009 };

        let a = array![[1, 2], [3, 4]];
        let b = array![[5, 6]];
        let c = array![[19, 22], [43, 50]];

        freivalds.verify(&a, &b, &c);
    }

    #[test]
    fn test_freivalds_verify_incorrect_result() {
        let freivalds = Freivalds { field: 1009 };

        let a = array![[1, 2], [3, 4]];
        let b = array![[5, 6], [7, 8]];
        let c = array![[18, 22], [43, 50]];

        assert!(!freivalds.verify(&a, &b, &c));
    }
}
