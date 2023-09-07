use std::fmt::Debug;

fn main() {
    let mut list = vec![1, 2, 3, 4, 5, 6];
    // println!("{:?}", list);
    
    rotate_list(&mut list, 4);

    println!("{:?}", list);
}

fn rotate_list<T: Debug>(list: &mut [T], rotate_by: usize) {
    let len = list.len();

    if rotate_by >= len {
        return;
    }
    
    reverse_list(list, 0, rotate_by - 1);
    // println!("{:?}", list);
    reverse_list(list, rotate_by, len - 1);
    // println!("{:?}", list);
    reverse_list(list, 0, len - 1);
    // println!("{:?}", list);
}


fn reverse_list<T>(list: &mut [T], start: usize, end: usize) {
    let mut i = start;
    let mut j = end;

    while i < j {
        list.swap(i, j);
        i += 1;
        j -= 1;
    }
}