pub fn get_odd_elements<T: Clone>(list: &[T]) -> Vec<T> {
    let mut odd_list = Vec::new();
    for (i , el) in list.iter().enumerate() {
        // for human 0th index would be 1, 1st would be 2 and so on
        if i % 2 == 0 {
            odd_list.push(el.clone());
        }
    }

    odd_list
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = vec![1, 2, 3, 4, 5, 6, 7];
        let result = get_odd_elements(&list1);

        assert_eq!(result, vec![1, 3, 5, 7]);

        let list2 = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"];
        let result = get_odd_elements(&list2);

        assert_eq!(result, vec!["A", "C", "E", "G", "I"]);
    }
}
