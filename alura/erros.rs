pub fn main() {
    // let v = vec![1, 2, 3];

    // v[4];

    // panic!("erro proposital");

    match result() {
        Ok(s) => println!("String de sucesso = {}", s),
        Err(numero) => println!("Codigo de erro = {}", numero),
    }
}

fn result() -> Result<String, u8> {
    // Ok(String::from("Tudo deu certo"))
    Err(42)
}
