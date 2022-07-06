pub fn main() {
    let titular = Titular {
        nome: String::from("Gabriel"),
        sobrenome: String::from("Oliveira"),
    };

    let mut conta: Conta = Conta {
        titular: titular,
        saldo: 100.0,
    };

    conta.sacar(10.0);

    println!(
        "Dados da conta: Nome titular = {}, Sobrenome titular = {}, Saldo = {}",
        conta.titular.nome, conta.titular.sobrenome, conta.saldo
    );
}

struct Conta {
    titular: Titular,
    saldo: f64,
}

impl Conta {
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
    }
}

struct Titular {
    nome: String,
    sobrenome: String,
}
