use std::fmt;

use chrono::{self, NaiveDate};
use serde::{Serialize, Deserialize};

pub mod core;
pub mod date;
pub mod errors;
pub mod screens;
pub mod validation;

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
    nome: String,
    id: u64,
    quantidade_estoque: u64,
    valor: f64,
    quantidade_restoque: u64,
    categoria: Categoria,
    #[serde(with = "date")]
    data_restoque: chrono::NaiveDate
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
        write!(f, "{}\nID: {}\nEstoque: {}\nPreço: R${:.2}\nMínimo para restoque: {}\nData do último restoque: {}\nCategoria: {}",
                self.nome, self.id, self.quantidade_estoque, self.valor, self.quantidade_restoque, self.data_restoque, self.categoria)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Venda {
    vendedor: String,
    produtos: Vec<u64>,
    codigo: u64,
    valor: f64,
    metodo_pagamento: MetodoPagamento,
    #[serde(with = "date")]
    data: chrono::NaiveDate
}

impl Venda {
    pub fn new(vendedor: String, codigo: u64, valor: f64, data: chrono::NaiveDate, metodo_pagamento: MetodoPagamento) -> Self {
        Venda {
            vendedor,
            produtos: Vec::new(),
            codigo,
            valor,
            data,
            metodo_pagamento
        }
    }
}

impl std::fmt::Display for Venda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Data da venda: {}\nVenda realizada por: {}\nCódigo: {}\nValor: R${:.2}\nMétodo de pagamento: {}\nProdutos vendidos:\n{:#?}",
                self.data, self.vendedor, self.codigo, self.valor, self.metodo_pagamento, self.produtos)
    }
}
