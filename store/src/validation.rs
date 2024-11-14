use super::{Produto, Categoria, MetodoPagamento, errors};
use std::{error::Error, fs::{File, OpenOptions}, process};

pub fn get_files() -> (File, File, File) {
    println!("Insira o caminho para o arquivo de armazenamento de produtos:");
    let products_file = get_file();

    println!("Insira o caminho para o arquivo de vendas:");
    let sales_file = get_file();

    println!("Insira o caminho para o arquivo índex de vendas:");
    let sales_index_file = get_file();

    (products_file, sales_file, sales_index_file)
}

fn get_file() -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(get_string())
        .unwrap_or_else(|error| {
            eprintln!("Ocorreu um erro tentando abrir o arquivo: {error}.");
            process::exit(1);
        })
}

pub fn get_option() -> u64 {
    loop {
        super::screens::menu_screen();

        let mut buf = String::new();
        if let Err(error) = std::io::stdin().read_line(&mut buf) {
            eprintln!("Ocorreu um erro ao tentar ler a opção selecionada: {error}.\nCertifique-se de ter inserido corretamente.");
            continue;
        };

        if buf.trim().to_lowercase() == "sair" {
            return 0;
        }

        let option: u64 = match buf.trim().parse() {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Ocorreu um erro ao tentar ler a opção selecionada: {error}.\nCertifique-se de ter inserido corretamente.");
                continue;
            }
        };

        return option;
    }
}

pub fn get_string() -> String {
    loop {
        let mut buf = String::new();
        if let Err(error) = std::io::stdin().read_line(&mut buf) {
            eprintln!("Um erro ocorreu na leitura: {error}.");
            continue;
        }

        return buf.trim().to_string();
    }
}

pub fn validate_string() -> Result<String, errors::CustomErrors> {
    loop {
        let mut buf = String::new();
        if let Err(error) = std::io::stdin().read_line(&mut buf) {
            eprintln!("Um erro ocorreu na leitura: {error}.");
            continue;
        }

        if buf.trim().to_lowercase() == "sair" {
            return Err(errors::CustomErrors::OperationCanceled);
        }

        return Ok(buf.trim().to_string());
    }
}

pub fn validate_id_search() -> Result<u64, errors::CustomErrors> {
    loop {
        println!("Digite o ID do produto que deseja procurar (ou sair para cancelar a operação):");

        let buf = validate_string()?;

        match validate_int(&buf) {
            Ok(id) => return Ok(id),
            Err(error) => eprintln!("Um erro ocorreu ao tentar converter o ID: {error}.\nCertifique-se de que um valor válido foi inserido.")
        };
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

pub fn validate_product(input: Vec<&str>) -> Result<Produto, Box<dyn Error>> {
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

pub fn validate_sale(string: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let info: Vec<&str> = string.split_whitespace().collect();
    let amount: u64;

    match info.len() {
        1 => amount = 1,
        2 => amount =  validate_int(info[1])?,
        _ => return Err(Box::new(errors::CustomErrors::TooManyArguments))
    }

    let value = validate_int(info[0])?;

    Ok((value, amount))
}

pub fn validate_payment_method() -> Result<MetodoPagamento, Box<dyn Error>> {
    println!("Insira a forma de pagamento:");

    let mut buf = String::new();

    std::io::stdin().read_line(&mut buf)?;
    
    let payment_method = match buf.trim().to_lowercase().as_str() {
        "credito" => MetodoPagamento::Credito,
        "debito" => MetodoPagamento::Debito,
        "pix" => MetodoPagamento::Pix,
        "dinheiro" => MetodoPagamento::Dinheiro,
        _ => {
            return Err(Box::new(errors::CustomErrors::NoCategory));
        }
    };

    return Ok(payment_method);
}

pub fn validate_date() -> Result<chrono::NaiveDate, errors::CustomErrors> {
    loop {
        println!("Digite a data de venda que deseja procurar seguindo o formato dd/mm/YYYY ou digite 'sair' para cancelar");

        let buf = validate_string()?;

        match chrono::NaiveDate::parse_from_str(&buf, "%d/%m/%Y") {
            Ok(date) => return Ok(date),
            Err(error) => eprintln!("Ocorreu um erro ao tentar ler a data informada: {error}.\nCertifique-se de que a data está inserida no formato correto.")
        }
    }
}