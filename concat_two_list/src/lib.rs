pub fn concat<T: Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut vec = Vec::new();

    for el in left.iter() {
        vec.push(el.clone());
    }

    for el in right.iter() {
        vec.push(el.clone())
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = vec![1, 2, 3, 4, 5];
        let list2 = vec![6, 7, 8, 9, 10];
        let result = concat(&list1, &list2);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
