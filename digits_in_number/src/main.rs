fn main() {
   let digits = get_digits_in_num(1987239847);

   for digit in digits {
    print!("{digit} ");
   }
}


fn get_digits_in_num(num: u128) -> Vec<u128> {
    let mut digits: Vec<u128> = Vec::new();
    let mut quotient = num;

    while quotient != 0 {
        let modulos = quotient % 10;
        quotient = quotient / 10;
        digits.push(modulos);
    }

    reverse_list(&mut digits);

    digits
}

fn reverse_list<T>(vec: &mut [T]) {
    let mut start = 0;
    let mut end = vec.len() - 1;

    while start < end {
        vec.swap(start, end);
        start+=1;
        end-=1;
    }
}