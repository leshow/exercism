pub struct PascalsTriangle {
    rows: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { rows: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.rows).map(|x| self.row(x)).collect()
    }
    fn row(&self, x: u32) -> Vec<u32> {
        let mut v = vec![1];

        for k in 0..x {
            let last = v[k as usize];
            v.push(last * (x - k) / (k + 1));
        }
        return v;
    }
}
