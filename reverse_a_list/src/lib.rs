use std::fmt::Debug;

pub fn reverse<T: Debug>(v: &mut [T]) {
    let size = v.len();
    let mid = size / 2;

    for i in 0..mid {
        v.swap(i, size - i - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list1 = [19, 48, 13, 85, 12, 92];
        reverse(&mut list1);

        assert_eq!(list1, [92, 12, 85, 13, 48, 19]);

        let mut list2 = [19, 48, 13, 85, 12, 92, 101, 21, 26];
        reverse(&mut list2);

        assert_eq!(list2, [26, 21, 101, 92, 12, 85, 13, 48, 19]);

        let mut list3: [u32; 0] = [];
        reverse(&mut list3);
        assert_eq!(list3, []);
    }
}
