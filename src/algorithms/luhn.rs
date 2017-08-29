#[derive(Clone)]
#[derive(Debug)]
pub struct CardNo {
    vec: Vec<u8>,
    checkdigit: u8,
    i: u8,
}

pub struct Card {
    data: CardNo,
    len: usize,
    missing: u8,
    pub valid: bool,
}
impl Card {
    pub fn new(s: &str) -> Card {
        let len = s.len() - 1;
        let mut a = CardNo::new(s);
        println!("\n\n{:?}\n\n", a);
        let valid = a.verify();
        Self {
            data: a,
            len,
            missing: 0,
            valid,
        }
    }
}
impl CardNo {
    pub fn new(s: &str) -> Self {
        let mut work: Vec<u8> = {
            let a = s.split_whitespace().map(|x| x.trim()).collect::<String>();
            a.chars().map(|x| x.to_digit(10).unwrap() as u8).collect()
        };

        Self {
            checkdigit: work.pop().unwrap(),
            vec: work,
            i: 0,
        }

    }
    pub fn total(&self) -> u32 {
        let copy = self.clone();
        let mut even = true;
        let mut total = 0;
        for i in copy.rev() {
            total += {
                if even {
                    let a = i * 2;
                    if a > 9 {
                        println!("{}", a - 9);
                        a - 9
                    } else {
                        println!("{}", a);
                        a
                    }
                } else {
                    println!("{}", i);
                    i
                }
            } as u32;
            even = !even;

        }
        println!("{}", total);
        total
    }

    pub fn verify(&self) -> bool {
        let total = self.total();
        let remainder = total * 9 % 10;
        println!("remainder = {}", remainder);
        if remainder as u8 == self.checkdigit {
            true
        } else {
            false
        }
    }
}

impl Iterator for CardNo {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.vec.len() as u8 - 1 > self.i {
            let a = Some(self.vec[self.i as usize]);
            self.i += 1;
            a
        } else {
            self.i = 0;
            None
        }
    }
}
impl DoubleEndedIterator for CardNo {
    fn next_back(&mut self) -> Option<u8> {
        self.vec.pop()
    }
}
