struct Neumann{
    num: u32,
    prev: Vec<u32>
}

impl Neumann {
    pub fn next(&mut self) {
        let a = self.num * self.num;
        let b = (a % 1_000_000) / 100;
        self.prev.push(self.num);
        self.num = b;
    }
    pub fn new(a: u32) -> Self {
        Self {
            num: a,
            prev: Vec::new()
        }
    }
    pub fn check_repeats(&self) -> bool {
        for i in 0..self.prev.len()
        {
            if self.num == self.prev[i] {
                return true
            }
        }
        false
    }
    pub fn get_len(&self) -> usize {
        self.prev.len()
    }
}
pub fn neumann_unique_test(i: u32) -> usize {
             let mut a = Neumann::new(i);
            while {
                !a.check_repeats()
            }
            {
                a.next();
            }
   a.get_len()
}

