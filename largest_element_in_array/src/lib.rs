pub fn get_largest_element<T: PartialOrd + Clone>(elements: &[T]) -> T {
    let mut largest = elements[0].clone();
    for element in elements {
        if element > &largest {
            largest = (*element).clone();
        }
    }

    return largest;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = [2, 3, 9, 10, 30, 23, 32, ];
        let result = get_largest_element(&list1);
        assert_eq!(result, 32);

        let list2 = ['A', 'B', 'C', 'D', 'F'];
        let result = get_largest_element(&list2);
        assert_eq!(result, 'F');
    }
}
