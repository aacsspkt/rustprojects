
fn main() {

    let mut list = Vec::new();

    for i in 0..20 {
        list.push(i);
    }

    let closure = |val| {
        println!("{} * {} = {}", val, val, val * val);
    };

    on_all(&list, closure);
}

fn on_all<'a, T>(list: &'a[T], closure: impl Fn(&'a T))
{
    for el in list.iter() {
        closure(el);
    }
}

