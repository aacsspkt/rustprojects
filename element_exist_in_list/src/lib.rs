// Write a function that checks whether an element occurs in a list.

pub fn exists<T: PartialEq>(list: &[T], element: T) -> bool {
    for el in list.iter() {
        if *el == element {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = vec![1, 3, 8, 2, 4, 9, 6, 5, 7, 10];
        let result = exists(&list1, 2);
        assert!(result);

        let list2 = vec!["Alish", "Subash", "Roshan", "Dhiran", "Amin", "Jabir", "Raghib"];
        let result = exists(&list2, "Ashish");
        assert!(!result);

    }
}
