use std::default::Default;
pub struct Fibonacci {
    num_prev: u32,
    num: u32,
    count: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let temp = self.num;
        self.num = self.num_prev + temp;
        self.num_prev = temp;
        self.count += 1;
        Some(self.num)

    }
}
impl Default for Fibonacci {
    fn default() -> Self {
        Fibonacci {
            num_prev: 0,
            num: 1,
            count: 1,
        }
    }
}

impl Fibonacci {
    pub fn new(a: u32, b: u32) -> Fibonacci {
        Fibonacci {
            num_prev: a,
            num: b,
            count: 1,
        }
    }
    pub fn get_nth_fibonacci(&mut self, c: u32) -> u32 {
        while self.count < c {
            self.next();
        }
        self.num
    }
    pub fn out_n_fibonacci(&mut self, c: u32) -> String {
        let mut out_string = String::new();

        while self.count <= c {
            out_string += &self.num.to_string();
            out_string += " ";
            self.next();

        }
        out_string
    }
    pub fn first_fibonacci_over(&mut self, v: u32) -> (u32, u32) {
        while self.num <= v {
            self.next();
        }
        (self.num, self.count)
    }
}
