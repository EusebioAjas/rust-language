fn main() {
    let a = 7;
    let a_box: Box<i32>;

    let mut a_ref: &i32 = &a;

    println!("{} {}", a, *a_ref);
    a_box = Box::new(a + 2);
    a_ref = &*a_box;
    println!(" {} {} {}", a, *a_ref, *a_box);
}
