pub fn main() {
    let mut a: Vec<u32> = Vec::with_capacity(10);

    a.insert(9, 9);

    println!("{}", a[9]);

    // for i in 0..10 {
    //     println!("{}", a[i]);
    // }
}
