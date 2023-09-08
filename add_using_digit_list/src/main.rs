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

    fn add(&self, other: &DigitList) -> DigitList {
        let mut result = Vec::new();
        let mut carry = 0;
        let max_len = std::cmp::max(self.digits.len(), other.digits.len());

        for i in 0..max_len {
            let sum = self.digit_at(i) + other.digit_at(i) + carry;
            carry = sum / 10;
            result.push(sum % 10)
        }

        if carry > 0 {
            result.push(carry);
        }

        DigitList { digits: result }
    }

    fn to_string(&self) -> String {
        self.digits.iter().rev().map(|d| d.to_string()).collect()
    }
}

fn main() {
    let digits1 = DigitList::new(vec![1, 5, 6]);

    let digits2 = DigitList::new(vec![9, 9]);

    let sum = digits1.add(&digits2);
    println!("{}", sum.to_string());
}
