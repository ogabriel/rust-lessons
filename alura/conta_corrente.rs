pub fn main() {
    let titular = Titular {
        nome: String::from("Gabriel"),
        sobrenome: String::from("Oliveira"),
    };

    let conta: Conta = Conta {
        titular: titular,
        saldo: 100.0,
    };

    println!(
        "Dados da conta: Nome titular = {}, Sobrenome titular = {}, Saldo = {}",
        conta.titular.nome, conta.titular.sobrenome, conta.saldo
    );
}

struct Conta {
    titular: Titular,
    saldo: f64,
}

struct Titular {
    nome: String,
    sobrenome: String,
}
