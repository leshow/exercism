use std::fmt;

const NUMERAL_MAP: [(usize, &'static str); 13] = [(1000, "M"),
                                                (900, "CM"),
                                                (500, "D"),
                                                (400, "CD"),
                                                (100, "C"),
                                                (90, "XC"),
                                                (50, "L"),
                                                (40, "XL"),
                                                (10, "X"),
                                                (9, "IX"),
                                                (5, "V"),
                                                (4, "IV"),
                                                (1, "I")];
pub struct Roman {
    count: usize
}

impl From<usize> for Roman {
    fn from(n: usize) -> Self {
        Roman::new(n)
    }
}

impl Roman {
    pub fn new(n: usize) -> Self {
        Roman {
            count: n
        }
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sum = self.count;
        let mut res = String::new();
        for &(val, numeral) in NUMERAL_MAP.iter() {
            while sum >= val {
                sum = sum - val;
                res.push_str(numeral);
            }
        }
        write!(f, "{}", res)
    }
}