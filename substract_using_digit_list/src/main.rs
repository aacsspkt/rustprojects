struct DigitList {
    digits: Vec<u8>,
}

impl DigitList {
    pub fn new(val: Vec<u8>) -> DigitList {
        let mut digits = val;
        digits.reverse();
        DigitList { digits }
    }

    fn digit_at(&self, index: usize) -> u8 {
        if index < self.digits.len() {
            self.digits[index]
        } else {
            0
        }
    }



    fn substract(&self, other: &DigitList) -> DigitList {
        let mut result = Vec::new();
        let mut borrow = 0;
        let max_len = std::cmp::max(self.digits.len(), other.digits.len());

        for i in 0..max_len {
            let diff = self.digit_at(i) as i32 - other.digit_at(i) as i32 - borrow; 

            if diff < 0 {
                borrow = 1;
                result.push((diff + 10) as u8);
            } else {
                borrow = 0;
                result.push(diff as u8);
            }
        }

        while let Some(&0) = result.last() {
            result.pop();
        }

        DigitList { digits: result }
    }

    fn to_string(&self) -> String {
        self.digits.iter().rev().map(|d| d.to_string()).collect()
    }
}

fn main() {
    
    let digits2 = DigitList::new(vec![9, 9]);
    let digits1 = DigitList::new(vec![1, 5, 6]);

    let diff = digits1.substract(&digits2);
    println!("{}", diff.to_string());
}
