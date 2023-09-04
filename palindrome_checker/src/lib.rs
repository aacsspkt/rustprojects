pub fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase();

    let input = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();

    let len = input.len();

    for i in 0..len / 2 {
        if input.chars().nth(i) != input.chars().nth(len - 1 - i) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man, a plan, a canal, Panama"));
        assert!(!is_palindrome("input"));
    }
}
