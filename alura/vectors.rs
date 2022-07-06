pub fn main() {
    // let mut notas: Vec<f32> = Vec::new();

    // notas.push(10.0);
    // notas.push(5.5);
    // notas.push(7.1);

    // println!("{:?}", notas);

    // notas.push(8.1);

    let notas: Vec<f32> = vec![10.0, 5.5, 7.1, 8.1];

    println!("{:?}", notas);
    println!("Nota 1 = {}", notas[0]);
    println!(
        "Nota 8 = {}",
        match notas.get(7) {
            Some(&n) => n,
            None => 0.0,
        }
    );
}
