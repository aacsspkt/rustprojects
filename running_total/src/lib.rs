use std::ops::Add;

pub fn running_total<T>(list: &[T]) -> Vec<T>
where T: Clone + Default + Add<Output = T>
{
    let mut result = Vec::new();
    let  mut running_sum  = T::default();

    for el in list.iter() {
        running_sum  = running_sum + el.clone();
        result.push(running_sum.clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = vec![1, 2, 1, 3, 2, 1];
        let result = running_total(&list1);
        assert_eq!(result, vec![1, 3, 4, 7, 9, 10]);
    }
}
