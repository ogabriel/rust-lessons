pub fn main() {
    let idade:u8 = 17;
    let responsavel_autorizou = true;

    if idade >= 18  {
        println!("Pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar na balada por autorização do responsável");
    } else {
        println!("Não pode entrar na balada");
    }
}
