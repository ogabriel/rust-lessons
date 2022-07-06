pub fn main() {
    let matriz: [[f32; 3]; 2] = [[0.0, 1.2, 0.1], [1.3, 0.3, 1.4]];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}
