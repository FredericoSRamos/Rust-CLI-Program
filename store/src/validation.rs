use super::Produto;
use super::Categoria;

pub fn validate_path(buf: &mut String) -> &str {
    loop {
        println!("Insira o caminho para o arquivo de armazenamento (ou 'sair' para cancelar a operação):");

        if let Err(error) = std::io::stdin().read_line(buf) {
            eprintln!("Um erro ocorreu ao tentar ler o caminho do arquivo: {error}.");
            continue;
        }

        if buf.trim().to_lowercase() == "sair" {
            std::process::exit(0);
        }

        return buf.trim();
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
            Err(error) => {
                eprintln!("Um erro ocorreu ao tentar converter o ID: {error}.\nCertifique-se de que um valor válido foi inserido.");
                continue;
            }
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

pub fn validate_input(input: Vec<&str>) -> Result<Produto, Box<dyn std::error::Error>> {
    let quantidade_estoque = validate_int(input[1])?;
    let valor = validate_float(input[2])?;
    let quantidade_restoque = validate_int(input[3])?;
    let data_restoque = chrono::NaiveDate::parse_from_str(input[4], "%d/%m/%Y")?;

    let categoria = match input[5] {
        "Eletronico" => Categoria::Eletronico,
        "Roupa" => Categoria::Roupa,
        "Alimento" => Categoria::Alimento,
        _ => {
            return Err(Box::new(super::CustomErrors::NoCategory));
        }
    };

    return Ok(Produto::new(input[0].to_string(), 0, quantidade_estoque, valor, quantidade_restoque, data_restoque, categoria));
}