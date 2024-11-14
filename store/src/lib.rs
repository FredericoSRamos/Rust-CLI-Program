use std::fmt;
use std::fs::File;
use std::io::{self, Seek};
use std::error::Error;


use chrono::{self, NaiveDate};

pub mod validation;
pub mod screens;

#[derive(Debug)]
pub enum Categoria {
    Eletronico,
    Roupa,
    Alimento,
    Geral
}

#[derive(Debug)]
pub enum MetodoPagamento {
    Credito,
    Debito,
    Pix,
    Dinheiro
}

#[derive(Debug)]
pub enum CustomErrors {
    NoCategory,
    IDNotFound
}

impl fmt::Display for CustomErrors {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomErrors::NoCategory => write!(format, "A categoria especificada não existe"),
            CustomErrors::IDNotFound => write!(format, "O ID especificado não foi encontrado")
        }
    }
}

impl Error for CustomErrors {}

#[derive(Debug)]
pub struct Produto {
    pub nome: String,
    pub id: u64,
    pub quantidade_estoque: u64,
    pub valor: f64,
    pub quantidade_restoque: u64,
    pub data_restoque: chrono::NaiveDate,
    pub categoria: Categoria
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

#[derive(Debug)]
pub struct Venda {
    pub produto: String,
    pub numero_produtos: u64,
    pub valor: f64,
    pub data_venda: chrono::NaiveDate,
    pub metodo_pagamento: MetodoPagamento
}

impl Venda {
    pub fn new(produto: String, numero_produtos: u64, valor: f64, data_venda: chrono::NaiveDate, metodo_pagamento: MetodoPagamento) -> Self {
        Venda {
            produto,
            numero_produtos,
            valor,
            data_venda,
            metodo_pagamento
        }
    }
}
pub fn get_option() -> Result<u64, Box<dyn Error>> {
    screens::menu_screen();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    if buf.trim().to_lowercase() == "sair" {
        return Ok(0);
    }

    let option: u64 = buf.trim().parse()?;
    return Ok(option);
}

pub fn add_product(file: &mut File) -> Result<(), Box<dyn Error>> {
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

        let product = match validation::validate_input(fields) {
            Ok(product) => product,
            Err(error) => {
                eprintln!("Um erro ocorreu durante a conversão de argumentos: {error}.\nVerifique se todos os campos foram inseridos corretamente.");
                continue;
            }
        };
        
          // Atualizar o id com base na posição do item no arquivo e retornar o id
    }
}

pub fn register_sale(file: &mut File) -> Result<(), Box<dyn Error>> {
    
    loop {
        screens::add_sale_screen();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        if buf.trim().to_lowercase() == "sair" {
            return Ok(());
        }

        let fields: Vec<&str> = buf.split(',').map(|field| field.trim()).collect();

        if fields.len() != 5 {
            eprintln!("Número insuficiente de argumentos.");
            continue;
        }

        let sale = match validation::validate_input_sale(fields) {
            Ok(sale) => sale,
            Err(error) => {
                eprintln!("Um erro ocorreu durante a conversão de argumentos: {error}.\nVerifique se todos os campos foram inseridos corretamente.");
                continue;
            }
        };

        return Ok(());
    }

    // Procurar no arquivo o produto pelo id - função search_id
    // inserir venda no arquivo de vendas
    // se nao encontrar
    //return Err(Box::new(CustomErrors::IDNotFound));
}

pub fn search_id(file: &mut File, id: u64, product: &mut Produto) -> Result<(), Box<dyn Error>> {

    // Retorna o produto caso encontrado com base no id (posição)
    // Retorna -1 caso não encontrado

    return Ok(());
}

pub fn search_product(file: &mut File, produto: &mut Produto) -> Result<(), Box<dyn Error>> {

    // Retorna o produto caso encontrado com base no nome
    // Retorna -1 caso não encontrado

    return Ok(());
}

pub fn products_needing_restock(file: &mut File) -> Result<(), Box<dyn Error>> {
    file.seek(io::SeekFrom::Start(0))?;

    return Ok(());
}


pub fn search_sale_date(file: &mut File, data: chrono::NaiveDate) -> Result<(), Box<dyn Error>> {

    // Retorna as vendas com esta data caso encontrado
    // Retorna -1 caso não encontrar

    return Ok(());
}

pub fn search_product_sales(file: &mut File) -> Result<(), Box<dyn Error>> {

    println!("insira nome do produto cujo interesse na vendas:");
    
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    buf.trim().to_string();

    //procurar no arquivo de vendas o produto
    //exibir todas suas vendas
    //Retornar -1 caso não encontrar venda

    return Ok(());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_product() {
        //add_product().unwrap();
    }
}
