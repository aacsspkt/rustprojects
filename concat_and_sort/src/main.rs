fn main() {
    let list1 = vec![1, 8, 10, 5, 6, 11, 99];
    let list2 = vec![17, 18, 12, 7, 10, 5];

    let list = concat_and_sort(&list1, &list2);

    for el in list {
        println!("{el}");
    }
}

fn concat_and_sort<T: Clone + PartialOrd>(list1: &[T], list2: &[T])-> Vec<T> {
    let mut list = concat(list1, list2);
    sort(&mut list);
    list
}

fn concat<T: Clone>(list1: &[T], list2: &[T]) -> Vec<T> {
    let mut vec = Vec::new();

    for element in list1 {
        vec.push(element.clone());
    }

    for element in list2 {
        vec.push(element.clone())
    }

    vec
}

fn sort<T: PartialOrd>(list: &mut [T]) {
     for i in 0..list.len() {
        let mut min_j = i;
        
        for j in (i+1)..list.len() {
            if list[j] < list[min_j] {
                min_j = j
            }
        }

        if min_j != i {
            list.swap(i, min_j)
        }
    }
}