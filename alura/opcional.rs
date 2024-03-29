pub fn main() {
    let conteudo_do_arquivo = ler_arquivo(String::from(""));

    match &conteudo_do_arquivo {
        Some(valor) => println!("{valor}"),
        None => println!("Não existe"),
    }

    println!("{:?}", conteudo_do_arquivo);

    if let Some(valor) = conteudo_do_arquivo {
        println!("{valor}");
    }
}

fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
    // None
}
