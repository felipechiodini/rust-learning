use std::io;

fn main() {
    println!("Digite algo:");

    // 1. Cria uma String vazia e mutável para armazenar o texto
    let mut linha = String::new();

    // 2. Lê a entrada do terminal e grava na variável 'linha'
    io::stdin()
        .read_line(&mut linha)
        .expect("Falha ao ler a linha");

    // 3. Remove a quebra de página (\n) automática do final e exibe
    println!("Você digitou: {}", linha.trim());
}
