pub fn main() {
    for i in 1..=20 {
        println!("{}: {}", i, match i {
            1 => "Pouco",
            2 | 3 => "Um pouquinho",
            4..=10 => "Um bocado",
            _ if i % 2 == 0 => "Uma boa quantidade",
            _ => "Muito"
        });
    }
}
