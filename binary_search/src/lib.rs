use std::cmp::Ordering;

pub fn search<T: Ord>(array: &[T], key: T)-> Option<usize> {
    let len = array.len();
    let mut left = 0;
    let mut right = len - 1;

    while left <= right {
        let mid = (left + right) / 2;
        match array[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid - 1
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = search(&list1, 9);

        assert_eq!(result, Some(8));

        let result = search(&list1, 20);
        assert_eq!(result, None);
    }
}