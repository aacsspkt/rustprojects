#[derive(Debug)]
struct DigitList {
    digits: Vec<u8>,
}

impl DigitList {
    pub fn new(value: Vec<u8>) -> DigitList {
        let mut digits = value;
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

    fn to_string(&self) -> String {
        self.digits.iter().rev().map(|d| d.to_string()).collect()
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

    fn multiply(&self, other: &DigitList) -> DigitList {
        let mut results: Vec<DigitList> = Vec::new();

        for i in 0..self.digits.len() {
            let mut result = vec![0; self.digits.len() + other.digits.len()];

            for j in 0..other.digits.len() {
                let product = (self.digit_at(i) as u64) * (other.digit_at(j) as u64);
                let position = i + j;
                let sum = result[position] as u64 + product;
                result[position] = (sum % 10) as u8;
                result[position + 1] = (sum / 10) as u8;
            }

            while let Some(&0) = result.last() {
                result.pop();
            }

            let digit_list = DigitList { digits: result };
            
            results.push(digit_list);
        }

        let mut sum = DigitList::new(vec![0]);
        
        for (i, digit_list) in results.iter().enumerate() {
            println!("{}: {}", i, digit_list.to_string());
            sum = sum.add(digit_list);
        }

        sum
    }
}

fn main() {
    let digit_list1 = DigitList::new(vec![3, 4, 8, 2]);
    let digit_list2 = DigitList::new(vec![1, 8, 9]);

    let product = digit_list1.multiply(&digit_list2);

    println!("The product is {}", product.to_string());
}
