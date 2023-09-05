use std::ops::Add;

pub fn sum_using_for_loop<T>(list: &[T]) -> T
where
    T: Clone + Default + Add<Output = T>,
{
    let mut sum = T::default();

    for el in list {
        sum = sum + el.clone()
    }

    sum
}

pub fn sum_using_while_loop<T>(list: &[T]) -> T
where
    T: Clone + Default + Add<Output = T>,
{
    let mut sum = T::default();
    let mut i = 0;
    while i < list.len() {
        sum = sum + list[i].clone();
        i += 1;
    }

    sum
}

pub fn sum_using_recursion<T>(list: &[T]) -> T
where 
    T: Clone + Default + Add<Output = T>
{
    match list {
        [] => T::default(),
        [first, rest @ .. ] => first.clone() + sum_using_recursion(rest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = vec![1, 2, 3, 4, 5];
        let result = sum_using_for_loop(&list1);
        assert_eq!(result, 15);

        let result = sum_using_while_loop(&list1);
        assert_eq!(result, 15);

        let result = sum_using_recursion(&list1);
        assert_eq!(result, 15);
    }
}
