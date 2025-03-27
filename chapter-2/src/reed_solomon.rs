pub struct Party {
    data: Vec<u64>,
}

impl Party {
    pub fn new(data: Vec<u64>) -> Self {
        Self { data }
    }

    pub fn evaluate(&self, r: u64, field: u64) -> u64 {
        let mut result = 0;
        let mut x = 1;

        for i in 0..self.data.len() {
            result += self.data[i] * x;
            x *= r;
        }

        result % field
    }
}

mod test {
    use crate::reed_solomon::Party;
    use rand::random;

    #[test]
    fn reed_solomon_example() {
        // Define data
        let data = vec![1, 2, 3, 4, 5];
        let n = data.len() as u64;

        // Create parties
        let alice = Party::new(data.clone());
        let bob = Party::new(data.clone());

        // Choose a random field and ensure it is larger than n^2
        let field = 31;
        assert!(field > n * n);

        // Choose a random r from the field
        let r = random::<u64>() % field;

        // Alice evaluates the polynomial at r
        let v = alice.evaluate(r, field);

        // By sending the value v to Bob, Bob evaluates the polynomial at r
        // and checks if the data is the same.
        // Note that Bob and Alice use the same hash function.
        assert_eq!(bob.evaluate(r, field), v);
    }
}
