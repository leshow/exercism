use std::collections::BTreeSet;

pub struct Triangle {
    sides: [i32; 3],
}

impl Triangle {
    pub fn build(input: [i32; 3]) -> Result<Triangle, &'static str> {
        if input.iter().any(|&size| size <= 0) {
            return Err("Illegal");
        }
        if input[0] + input[1] <= input[2] || input[0] >= input[1] + input[2] {
            return Err("Illegal");
        }
        Ok(Triangle { sides: input })
    }
    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().collect::<BTreeSet<_>>().len() == 1
    }
    pub fn is_isosceles(&self) -> bool {
        self.sides.iter().collect::<BTreeSet<_>>().len() == 2
    }
    pub fn is_scalene(&self) -> bool {
        self.sides.iter().collect::<BTreeSet<_>>().len() == 3
    }
}
