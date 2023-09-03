use std::cmp::Ordering;

pub fn get_largest_element<T: PartialOrd + Clone>(elements: &[T]) -> Option<T> {
    if elements.is_empty() {
        return None; // Handle the case of an empty slice
    }

    let mut largest = elements[0].clone();

    for element in elements.iter().skip(1) {
        if let Ordering::Greater = element.partial_cmp(&largest).unwrap_or(Ordering::Equal) {
            largest = element.clone();
        }
    }

    return Some(largest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = [2, 3, 9, 10, 30, 23, 32];
        let result = get_largest_element(&list1);
        assert_eq!(result, Some(32));
        
        let list2 = ['A', 'B', 'C', 'D', 'F'];
        let result = get_largest_element(&list2);
        assert_eq!(result, Some('F'));
        
        let list3: [i32; 0] = [];
        let result = get_largest_element(&list3);
        assert_eq!(result, None);
    }
}
