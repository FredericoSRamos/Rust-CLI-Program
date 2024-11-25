use super::{Produto, Categoria, MetodoPagamento, errors};
use std::{error::Error, fs::{File, OpenOptions}, io::{self, BufRead}, process};

pub fn get_files() -> (File, File) {
    (
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("produtos.bin")
        .unwrap_or_else(|error| {
            eprintln!("Ocorreu um erro tentando abrir o arquivo: {error}");
            process::exit(1);
        }),
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("vendas.bin")
        .unwrap_or_else(|error| {
            eprintln!("Ocorreu um erro tentando abrir o arquivo: {error}");
            process::exit(1);
        })
    )
}

pub fn get_option() -> u64 {
    loop {
        super::screens::menu_screen();

        let mut buf = String::new();
        if let Err(error) = io::stdin().read_line(&mut buf) {
            eprintln!("Ocorreu um erro ao tentar ler a opção selecionada: {error}\nCertifique-se de ter inserido corretamente.");
            continue;
        };

        if buf.trim().to_lowercase() == "sair" {
            return 0;
        }

        let option: u64 = match buf.trim().parse() {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Ocorreu um erro ao tentar ler a opção selecionada: {error}\nCertifique-se de ter inserido corretamente.");
                continue;
            }
        };

        return option;
    }
}

pub fn validate_string<R: BufRead>(reader: &mut R) -> Result<String, errors::CustomErrors> {
    loop {
        let mut buf = String::new();
        if let Err(error) = reader.read_line(&mut buf) {
            eprintln!("Um erro ocorreu na leitura: {error}");
            continue;
        }

        if buf.trim().to_lowercase() == "sair" {
            return Err(errors::CustomErrors::OperationCanceled);
        }

        return Ok(buf.trim().to_string());
    }
}

pub fn validate_search<R: BufRead>(search: &str, reader: &mut R) -> Result<u64, errors::CustomErrors> {

    match search {
        "id" => println!("Digite o ID do produto (ou sair para cancelar a operação):"),
        _ => println!("Digite o código da venda (ou sair para cancelar a operação):")
    }

    loop {
        let buf = validate_string(reader)?;

        match validate_int(&buf) {
            Ok(id) => return Ok(id),
            Err(error) => eprintln!("Um erro ocorreu ao tentar converter o ID: {error}\nCertifique-se de que um valor válido foi inserido.")
        };
    }
}

fn validate_int(string: &str) -> Result<u64, std::num::ParseIntError> {
    let number = string.parse::<u64>()?;
    return Ok(number);
}

fn validate_float(string: &str) -> Result<f64, std::num::ParseFloatError> {
    let number = string.parse::<f64>()?;
    return Ok(number);
}

pub fn get_product_info<R: BufRead>(reader: &mut R) -> Result<Produto, Box<dyn Error>> {
    loop {
        super::screens::add_product_screen();

        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        if buf.trim().to_lowercase() == "sair" {
            return Err(Box::new(errors::CustomErrors::OperationCanceled));
        }

        let fields: Vec<&str> = buf.split(' ').map(|field| field.trim()).collect();

        if fields.len() != 6 {
            eprintln!("Número incorreto de argumentos.");
            continue;
        }

        match validate_product(fields) {
            Ok(product) => return Ok(product),
            Err(error) => eprintln!("Um erro ocorreu durante a conversão de argumentos: {error}\nVerifique se todos os campos foram inseridos corretamente.")
        };
    }
}

fn validate_product(input: Vec<&str>) -> Result<Produto, Box<dyn Error>> {
    if input[0].len() > 40 {
        return Err(Box::new(errors::CustomErrors::NameTooLong));
    }

    let quantidade_estoque = validate_int(input[1])?;
    let valor = validate_float(input[2])?;
    let quantidade_restoque = validate_int(input[3])?;
    let data_restoque = chrono::NaiveDate::parse_from_str(input[4], "%d/%m/%Y")?;

    let categoria = match input[5].to_lowercase().as_str() {
        "eletronico" => Categoria::Eletronico,
        "roupa" => Categoria::Roupa,
        "alimento" => Categoria::Alimento,
        "geral" => Categoria::Geral,
        _ => {
            return Err(Box::new(errors::CustomErrors::NoCategory));
        }
    };

    return Ok(Produto::new(input[0].to_string(), 0, quantidade_estoque, valor, quantidade_restoque, data_restoque, categoria));
}

pub fn get_sale_info<R: BufRead>(reader: &mut R) -> Result<(chrono::NaiveDate, MetodoPagamento), Box<dyn Error>> {
    println!("Digite a data da venda seguindo o formato dd/mm/YYYY (ou digite 'sair' para cancelar).\n");

    let date = validate_date(reader)?;
    let payment_method = validate_payment_method(reader)?;

    return Ok((date, payment_method));
}

pub fn validate_sale(string: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let info: Vec<&str> = string.split_whitespace().collect();
    let amount: u64;

    match info.len() {
        1 => amount = 1,
        2 => amount = validate_int(info[1])?,
        _ => return Err(Box::new(errors::CustomErrors::TooManyArguments))
    }

    let id = validate_int(info[0])?;

    Ok((id, amount))
}

pub fn validate_payment_method<R: BufRead>(reader: &mut R) -> Result<MetodoPagamento, Box<dyn Error>> {
    
    println!("\nInsira a forma de pagamento:\n
    Opções: credito, debito, pix, dinheiro
    \n* Atenção: Não utilizar acento! *\n");

    let mut buf = String::new();

    reader.read_line(&mut buf)?;
    
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

pub fn validate_date<R: BufRead>(reader: &mut R) -> Result<chrono::NaiveDate, errors::CustomErrors> {
    loop {
        let buf = validate_string(reader)?;

        match chrono::NaiveDate::parse_from_str(&buf, "%d/%m/%Y") {
            Ok(date) => return Ok(date),
            Err(error) => eprintln!("Ocorreu um erro ao tentar ler a data informada: {error}\nCertifique-se de que a data está inserida no formato correto.")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Cursor};

    use super::*;

    #[test]
    fn test_get_files() {
        get_files();

        assert!(fs::exists("produtos.bin").expect("Erro ao tentar localizar o arquivo."));
        assert!(fs::exists("vendas.bin").expect("Erro ao tentar localizar o arquivo."));
    }

    #[test]
    fn test_validate_string() {
        let input = b"Carlos";
        let mut cursor = Cursor::new(input);

        let result = validate_string(&mut cursor);

        assert!(result.is_ok());

        let input = b"sair";
        let mut cursor = Cursor::new(input);

        let result = validate_string(&mut cursor);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_search() {
        let input = b"3";
        let mut cursor = Cursor::new(input);

        let result = validate_search("id", &mut cursor);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn test_validate_int() {
        let int = "10";

        let result = validate_int(int);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_validate_float() {
        let int = "5";

        let result = validate_float(int);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_get_product_info() {
        let input = b"Camisa 10 50 5 10/8/2023 Geral";
        let mut cursor = Cursor::new(input);

        let result = get_product_info(&mut cursor);

        assert!(result.is_ok());

        let produto = result.unwrap();

        assert_eq!(produto.nome, "Camisa");
        assert_eq!(produto.quantidade_estoque, 10);
        assert_eq!(produto.quantidade_restoque, 5);
    }

    #[test]
    fn test_validate_product() {
        let input = vec!["Camisa", "10", "35", "5", "15/11/2024", "eletronico"];

        let result = validate_product(input);

        assert!(result.is_ok());

        let produto = result.unwrap();

        assert_eq!(produto.nome, "Camisa");
        assert_eq!(produto.quantidade_estoque, 10);
        assert_eq!(produto.quantidade_restoque, 5);
    }

    #[test]
    fn test_get_sale_info() {
        let input = b"1/1/1970\ncredito";
        let mut cursor = Cursor::new(input);

        let result = get_sale_info(&mut cursor);

        assert!(result.is_ok());

        let sale_info = result.unwrap();

        assert_eq!(sale_info.0, chrono::NaiveDate::default());
    }

    #[test]
    fn test_validate_sale() {
        let result = validate_sale("2 3");

        assert!(result.is_ok());

        assert_eq!((2, 3), result.unwrap());
    }

    #[test]
    fn test_validate_payment_method() {
        let input = b"credito";
        let mut cursor = Cursor::new(input);

        let result = validate_payment_method(&mut cursor);

        assert!(result.is_ok());

        let input = b"invalido";
        let mut cursor = Cursor::new(input);

        let result = validate_payment_method(&mut cursor);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_date() {
        let input = b"1/1/1970";
        let mut cursor = Cursor::new(input);

        let result = validate_date(&mut cursor);

        assert!(result.is_ok());

        assert_eq!(result.unwrap(), chrono::NaiveDate::default());
    }
}