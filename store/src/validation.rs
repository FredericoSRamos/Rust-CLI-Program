use super::{Produto, Categoria, MetodoPagamento, errors};
use std::error::Error;

pub fn get_option() -> Result<u64, Box<dyn Error>> {
    super::screens::menu_screen();

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    if buf.trim().to_lowercase() == "sair" {
        return Ok(0);
    }

    let option: u64 = buf.trim().parse()?;
    return Ok(option);
}

pub fn get_path() -> String {
    loop {
        println!("Insira o caminho para o arquivo de armazenamento (ou 'sair' para cancelar a operação):");

        let mut buf = String::new();

        if let Err(error) = std::io::stdin().read_line(&mut buf) {
            eprintln!("Um erro ocorreu ao tentar ler o caminho do arquivo: {error}.");
            continue;
        }

        if buf.trim().to_lowercase() == "sair" {
            std::process::exit(0);
        }

        return buf.trim().to_string();
    }
}

pub fn set_seller() -> String {
    loop {
        println!("Insira o caixa que está realizando as vendas (ou 'sair' para cancelar a operação):");

        let mut buf = String::new();

        if let Err(error) = std::io::stdin().read_line(&mut buf) {
            eprintln!("Um erro ocorreu ao tentar ler o caminho do arquivo: {error}.");
            continue;
        }

        if buf.trim().to_lowercase() == "sair" {
            std::process::exit(0);
        }

        return buf.trim().to_string();
    }
}

pub fn validate_id_search() -> u64 {
    loop {
        println!("Digite o ID do produto que deseja procurar (ou sair para cancelar a operação):");

        let mut buf = String::new();
        if let Err(error) = std::io::stdin().read_line(&mut buf) {
            eprintln!("Um erro ocorreu ao tentar ler o ID desejado: {error}.");
            continue;
        }

        if buf.trim().to_lowercase() == "sair" {
            std::process::exit(0);
        }

        match validate_int(buf.trim()) {
            Ok(id) => return id,
            Err(error) => eprintln!("Um erro ocorreu ao tentar converter o ID: {error}.\nCertifique-se de que um valor válido foi inserido.")
        };
    }
}

pub fn validate_str_search() -> String {
    loop {
        println!("Digite o nome do produto que deseja procurar (ou sair para cancelar a operação):");

        let mut buf = String::new();
        if let Err(error) = std::io::stdin().read_line(&mut buf) {
            eprintln!("Um erro ocorreu ao tentar ler o nome desejado: {error}.");
            continue;
        }

        if buf.trim().to_lowercase() == "sair" {
            std::process::exit(0);
        }

        return buf.trim().to_string();
    }
}

pub fn validate_int(string: &str) -> Result<u64, std::num::ParseIntError> {
    let number = string.parse::<u64>()?;
    return Ok(number);
}

pub fn validate_float(string: &str) -> Result<f64, std::num::ParseFloatError> {
    let number = string.parse::<f64>()?;
    return Ok(number);
}

pub fn validate_input(input: Vec<&str>) -> Result<Produto, Box<dyn Error>> {
    if input[0].len() > 40 {
        return Err(Box::new(errors::CustomErrors::NameTooLong));
    }

    let quantidade_estoque = validate_int(input[1])?;
    let valor = validate_float(input[2])?;
    let quantidade_restoque = validate_int(input[3])?;
    let data_restoque = chrono::NaiveDate::parse_from_str(input[4], "%d/%m/%Y")?;

    let categoria = match input[5] {
        "Eletronico" => Categoria::Eletronico,
        "Roupa" => Categoria::Roupa,
        "Alimento" => Categoria::Alimento,
        "Geral" => Categoria::Geral,
        _ => {
            return Err(Box::new(errors::CustomErrors::NoCategory));
        }
    };

    return Ok(Produto::new(input[0].to_string(), 0, quantidade_estoque, valor, quantidade_restoque, data_restoque, categoria));
}

pub fn validate_payment_method() -> Result<MetodoPagamento, Box<dyn Error>> {
    println!("Insira a forma de pagamento:");

    let mut buf = String::new();

    std::io::stdin().read_line(&mut buf)?;
    
    let metodo_pagamento = match buf.trim().to_lowercase().as_str() {
        "credito" => MetodoPagamento::Credito,
        "debito" => MetodoPagamento::Debito,
        "pix" => MetodoPagamento::Pix,
        "dinheiro" => MetodoPagamento::Dinheiro,
        _ => {
            return Err(Box::new(errors::CustomErrors::NoCategory));
        }
    };

    return Ok(metodo_pagamento);
}

pub fn validate_date() -> chrono::NaiveDate {
    loop {
        println!("Digite a data de venda que deseja procurar seguindo o formato dd/mm/YYYY ou digite 'sair' para cancelar");

        let mut buf = String::new();
        if let Err(error) = std::io::stdin().read_line(&mut buf) {
            eprintln!("Ocorreu um erro ao tentar ler a data inserida {error}.");
            continue;
        };

        if buf.trim().to_lowercase() == "sair" {
            std::process::exit(0);
        }

        match chrono::NaiveDate::parse_from_str(buf.trim(), "%d/%m/%Y") {
            Ok(date) => return date,
            Err(error) => eprintln!("Ocorreu um erro ao tentar ler a data informada: {error}.\nCertifique-se de que a data está inserida no formato correto.")
        }
    }
}
