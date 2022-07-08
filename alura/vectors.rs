pub fn main() {
    // let mut notas: Vec<f32> = Vec::new();

    // notas.push(10.0);
    // notas.push(5.5);
    // notas.push(7.1);

    // println!("{:?}", notas);

    // notas.push(8.1);

    // let mut notas: Vec<f32> = vec![10.0, 5.5, 7.1];

    let mut notas: Vec<f32> = Vec::with_capacity(4);

    notas.push(10.0);
    notas.push(5.4);
    notas.push(7.1);

    println!("Capacidade = {}, Tamanho = {}", notas.capacity(), notas.len());

    notas.push(8.1);

    println!("Capacidade = {}, Tamanho = {}", notas.capacity(), notas.len());

    println!("{:?}", notas);
    println!("Nota 1 = {}", notas[0]);
    println!(
        "Nota 8 = {}",
        match notas.get(7) {
            Some(n) => *n,
            None => 0.0,
        }
    );

    // if let Some(nota) = notas.pop() {
    //     println!("Ultimo valor {}", nota);
    //     println!("{:?}", notas);
    // }

    // while let Some(nota) = notas.pop() {
    //     println!("Valor removido = {}", nota);
    // }

    for nota in &notas {
        println!("Valor = {}", nota);
    }

    println!("{:?}", notas);
}
