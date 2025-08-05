pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|&x| x <= 0) {
            return None;
        }
        let a = sides[0];
        let b = sides[1];
        let c = sides[2];
        if [(a + b) < c, (b + c) < a, (a + c) < b].iter().any(|&x| x) {
            return None;
        }
        Some(Self{a, b, c})
    }
    
    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c && self.a != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.a == self.c
    }
}
