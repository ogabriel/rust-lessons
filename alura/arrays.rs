pub fn main() {
    // let notas:[f32; 4] = [10f32, 8f32, 9.5, 6.0];
    let notas:[f32; 4] = [6.5; 4];

    println!("--interar--");

    for nota in &notas {
        println!("A nota é = {}", nota);
    }

    println!("--indice--");

    for i in 0..notas.len() {
        println!("A nota {} é = {}", i + 1, notas[i]);
    }

    println!("--endereço--");

    let inteiro:usize = 0;
    println!("acessando nota por inteiro {}", notas[inteiro]);
}
