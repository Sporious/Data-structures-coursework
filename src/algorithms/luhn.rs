pub struct Card {
    pub vec: Vec<u8>,
    pub missing: isize,
}

impl Card {
    pub fn output(&self) -> String {
        let mut string = String::new();
        for i in 0..self.vec.len() {
            string += &self.vec[i].to_string();
        }
        string
    }
    pub fn new(s: &str) -> Self {
        let mut v: Vec<u8> = Vec::new();
        let mut missing_pos = -1;
        for (index, i) in s.chars().enumerate() {
            match i.to_digit(10) {
                Some(num) => v.push(num as u8),
                None => {
                    v.push(0_u8);
                    missing_pos = index as isize;
                }
            }
        }
        Self {
            vec: v,
            missing: missing_pos,
        }
    }
    pub fn verify(&self) -> bool {
        check(&self.vec)
    }
    pub fn heal(&mut self) {
        if self.missing != -1 {
            let mut a = self.vec.clone();
            while {
                !check(&a)
            }
            {
                a[self.missing as usize] += 1;
            }
            self.missing = -1;
            self.vec = a;
        }

    }
    pub fn correct(&mut self) {
        if !self.verify() {
            let mut a = self.vec.clone();
            for i in 1..a.len() {
                a.swap(i - 1, i);
                if check(&a) {
                    break;
                } else {
                    a.swap(i - 1, i);
                }
            }
            self.vec = a;
        }
    }
}
pub fn check(v: &[u8]) -> bool {
    let total = total(v);
    //if total % 10 == 0 { true } else { false }
    total % 10 == 0
}
pub fn total(v: &[u8]) -> u32 {
    let mut total = 0;
    let mut even = false;
    let len = v.len();
    for i in 0..len {
        total += {
            if even {
                let a = v[len - 1 - i] * 2;
                if a > 9 { a - 9 } else { a }
            } else {
                v[len - 1 - i]
            }
        } as u32;
        even = !even;
    }
    total
}
