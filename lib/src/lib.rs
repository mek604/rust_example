
pub mod math {

    fn xy_gcd(mut n: u64, mut m: u64) -> u64 {
         // Rust always checks assertion!
         // debug_assert! is skipped when compiled
        assert!(n != 0 && m != 0);
        while m != 0 {
            if m < n {
                let t = m; // type inference, can only happen within function body
                m = n;
                n = t;
            }
            m = m % n;
        }
        n 
    }

    pub fn gcd(numbers: &Vec<u64>) -> u64 {
        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = xy_gcd(d, *m);
        }
        d
    }

}
