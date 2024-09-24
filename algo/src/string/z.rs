pub trait ZFunction {
    fn z_function(&self) -> Vec<u32>;
}

impl ZFunction for String {
    fn z_function(&self) -> Vec<u32> {
        let mut z = vec![0; self.len()];
        let mut l = 0;
        let mut r = 0;
        let bytes = self.as_bytes();
        for i in 1..bytes.len() {
            if i < r {
                z[i] = (r - i).min(z[i - l]);
            }
            while i + z[i] < self.len() && bytes[z[i]] == bytes[i + z[i]] {
                z[i] += 1;
            }

            if i + z[i] > r {
                l = i;
                r = i + z[i];
            }
        }
        z.into_iter().map(|v| v as u32).collect()
    }
}
