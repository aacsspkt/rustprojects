// Write a function that combines two lists by alternatingly taking elements, 
// e.g. [a,b,c], [1,2,3] → [a,1,b,2,c,3].
pub fn concat<T:Clone>(first: &[T], second: &[T]) -> Option<Vec<T>> {
    let mut vec = Vec::new();

    if first.len() != second.len() {
        return None;
    }

    for i in 0..first.len() {
        vec.push(first[i].clone());
        vec.push(second[i].clone());
    }

    Some(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = vec![1, 2, 3, 4, 5];
        let list2 = vec![6, 7, 8, 9, 10];
        let result = concat(&list1, &list2);
        assert_eq!(result, Some(vec![1, 6, 2, 7, 3, 8, 4, 9, 5, 10]));
    }
}
