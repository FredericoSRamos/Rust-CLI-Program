use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::error::Error;
use errors::CustomErrors;
use serde::{Serialize, Deserialize};
use validation::validate_int;
use std::fmt;

use chrono::{self, NaiveDate};

pub mod validation;
pub mod screens;
pub mod date;
pub mod errors;

const PRODUCT_LENGTH: usize = 102;

#[derive(Serialize, Deserialize)]
pub enum Categoria {
    Eletronico,
    Roupa,
    Alimento,
    Geral
}

impl std::fmt::Display for Categoria {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Categoria::Alimento => write!(f, "Alimento"),
            Categoria::Eletronico => write!(f, "Eletrônico"),
            Categoria::Roupa => write!(f, "Roupa"),
            Categoria::Geral => write!(f, "Geral")
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum MetodoPagamento {
    Credito,
    Debito,
    Pix,
    Dinheiro
}

impl std::fmt::Display for MetodoPagamento {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MetodoPagamento::Credito => write!(f, "Cartão de crédito"),
            MetodoPagamento::Debito => write!(f, "Cartão de débito"),
            MetodoPagamento::Dinheiro => write!(f, "Dinheiro"),
            MetodoPagamento::Pix => write!(f, "PIX")
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Produto {
    pub nome: String,
    pub id: u64,
    pub quantidade_estoque: u64,
    pub valor: f64,
    pub quantidade_restoque: u64,
    pub categoria: Categoria,
    #[serde(with = "date")]
    pub data_restoque: chrono::NaiveDate
}

impl Produto {
    pub fn new(nome: String, id: u64, quantidade_estoque: u64, valor: f64, quantidade_restoque: u64, data_restoque: chrono::NaiveDate, categoria: Categoria) -> Self {
        Produto {
            nome,
            id,
            quantidade_estoque,
            valor,
            quantidade_restoque,
            data_restoque,
            categoria
        }
    }

    pub fn default() -> Self {
        Produto {
            nome: String::new(),
            id: 0,
            quantidade_estoque: 0,
            valor: 0.0,
            quantidade_restoque: 0,
            data_restoque: NaiveDate::default(),
            categoria: Categoria::Geral
        }
    }
}

impl std::fmt::Display for Produto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\nID: {}\nEstoque: {}\nPreço: {}\nMínimo para restoque: {}\nData do último restoque: {}\nCategoria: {}",
                self.nome, self.id, self.quantidade_estoque, self.valor, self.quantidade_restoque, self.data_restoque, self.categoria)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Venda {
    vendedor: String,
    produtos: Vec<u64>,
    numero_produtos: u64,
    valor: f64,
    metodo_pagamento: MetodoPagamento,
    #[serde(with = "date")]
    data: chrono::NaiveDate
}

impl Venda {
    pub fn new(vendedor: String, numero_produtos: u64, valor: f64, data: chrono::NaiveDate, metodo_pagamento: MetodoPagamento) -> Self {
        Venda {
            vendedor,
            produtos: Vec::new(),
            numero_produtos,
            valor,
            data,
            metodo_pagamento
        }
    }
}

pub fn add_product(file: &mut File) -> Result<(), Box<dyn Error>> {
    let mut product;

    loop {
        screens::add_product_screen();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        if buf.trim().to_lowercase() == "sair" {
            return Ok(());
        }

        let fields: Vec<&str> = buf.split(',').map(|field| field.trim()).collect();

        if fields.len() != 6 {
            eprintln!("Número insuficiente de argumentos.");
            continue;
        }

        match validation::validate_input(fields) {
            Ok(value) => {
                product = value;
                break;
            },
            Err(error) => {
                eprintln!("Um erro ocorreu durante a conversão de argumentos: {error}.\nVerifique se todos os campos foram inseridos corretamente.");
                continue;
            }
        };
    }

    let track = PRODUCT_LENGTH as i64;
    let a = file.seek(io::SeekFrom::End(-track));

    if a.is_ok() {
        let mut buf = [0; PRODUCT_LENGTH];
        file.read_exact(&mut buf)?;
        let temp_product: Produto = bincode::deserialize(&buf)?;
        product.id = temp_product.id + 1;
        file.seek(SeekFrom::End(0))?;
    } else {
        product.id = 1;
    }

    let mut serialized = bincode::serialize(&product)?;
    serialized.resize(PRODUCT_LENGTH, 0);
    file.write(&serialized)?;

    println!("Produto adicionado com sucesso com o id {}.", product.id);

    Ok(())
}

pub fn register_sale(file: &mut File, seller: String) -> Result<(), Box<dyn Error>> {
    
    screens::add_sale_screen();
    let mut products: Vec<u64> = Vec::new();

    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        match buf.trim().to_lowercase().as_str() {
            "sair" => return Ok(()),
            "concluir" => break,
            _ => {}
        }

        let id = validate_int(buf.trim())?;

        products.push(id);
    }
    let mut product = Produto::default();
    let mut valor: f64 = 0.0;

    for id in products.iter() {
        search_product_id(file, *id, &mut product)?;

        valor += product.valor;

        if product.quantidade_estoque > 0 {
            product.quantidade_estoque -= 1;
        } else {
            return Err(Box::new(CustomErrors::EmptyStock));
        }

        let position = PRODUCT_LENGTH as u64 * (id - 1);
        file.seek(SeekFrom::Start(position))?;

        let mut serialized = bincode::serialize(&product)?;
        serialized.resize(PRODUCT_LENGTH, 0);
        file.write(&serialized)?;
    }

    let date = chrono::Local::now().date_naive();

    let sale = Venda::new(seller, products.len() as u64, valor, date, validation::validate_payment_method()?);
    
    let serialized = bincode::serialize(&sale)?;
    let serialized_size = bincode::serialize(&(serialized.len() as u64))?;

    file.seek(SeekFrom::End(0))?;
    file.write(&serialized_size)?;
    file.write(&serialized)?;

    Ok(())
}

pub fn search_product_id(file: &mut File, id: u64, product: &mut Produto) -> Result<(), Box<dyn Error>> {

    let position = PRODUCT_LENGTH as u64 * (id - 1);
    file.seek(SeekFrom::Start(position))?;

    let mut buf = [0; PRODUCT_LENGTH];
    file.read_exact(&mut buf)?;
    *product = bincode::deserialize(&buf)?;

    return Ok(());
}

pub fn search_product_name(file: &mut File, name: String, product: &mut Produto) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; PRODUCT_LENGTH];

    file.seek(SeekFrom::Start(0))?;

    loop {
        file.read_exact(&mut buf)?;

        *product = bincode::deserialize(&buf)?;

        if product.nome == name {
            println!("Produto encontrado.");
            return Ok(());
        }
    }
}

pub fn products_needing_restock(file: &mut File) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; PRODUCT_LENGTH];
    let mut product: Produto;

    file.seek(SeekFrom::Start(0))?;

    loop {
        file.read_exact(&mut buf)?;

        product = bincode::deserialize(&buf)?;

        if product.quantidade_estoque < product.quantidade_restoque {
            println!("{product}");
        }
    }
}

pub fn search_sale_date(file: &mut File, date: chrono::NaiveDate) -> Result<(), Box<dyn Error>> {

    let mut buf = [0; 8];
    file.read_exact(&mut buf)?;
    let size: u64 = bincode::deserialize(&buf)?;
    // Falta implementar
    //file

    //file.read_exact(&mut a)?;

    return Ok(());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[ignore = "reason"]
    fn test_product() {
        //add_product().unwrap();
    }
}
