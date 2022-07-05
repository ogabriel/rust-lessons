pub fn main() {
    let idade:u8 = 17;

    let condicao = if idade >= 18 { "maior" } else { "menor" };

    println!("É {} de idade", condicao);
}
