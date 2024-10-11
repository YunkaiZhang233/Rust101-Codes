fn main() {
    println!("Hello, world!");
}

fn init() {
    let a: f32 = 1.5;
    let b: bool = true;
    let ab: (f32, bool) = (a, b);
    let l: [i32; 5] = [0, 1, 2, 3, 4];
}

fn functional() {
    let foo = if 2 > 1 {
        1
    } else {
        let g = 2;
        g + 1
    };
}

struct Point_v1 {
    x: f32,
    y: f32,
}

struct Point_v2(f32, f32);

fn copy_example() {
    let a = 1;
    let b = a;
    println!("a: {}, b: {}", a, b);
}

struct Product<X, Y> {
    x: X,
    y: Y,
}

enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

enum Option<T> {
    Some(T),
    None,
}

fn mut_example() {
    let mut a = 1;
    a = 2;
    let mut_a = &mut a;
    // ERROR: cannot borrow `a` as mutable because it is also borrowed as immutable
    // let ref_a = &a;
    *mut_a = 3;
}