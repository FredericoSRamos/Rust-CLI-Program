use std::fmt;

use chrono;
use serde::{Serialize, Deserialize};

pub mod core;
pub mod date;
pub mod errors;
pub mod screens;
pub mod validation;

#[derive(Serialize, Deserialize)]
enum Categoria {
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
    fn new(nome: String, id: u64, quantidade_estoque: u64, valor: f64, quantidade_restoque: u64, data_restoque: chrono::NaiveDate, categoria: Categoria) -> Self {
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
}

impl std::fmt::Display for Produto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\nID: {}\nEstoque: {}\nPreço: R${:.2}\nMínimo para restoque: {}\nData do último restoque: {}\nCategoria: {}",
                self.nome, self.id, self.quantidade_estoque, self.valor, self.quantidade_restoque, self.data_restoque.format("%d/%m/%Y"), self.categoria)
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
    fn new(vendedor: String, codigo: u64, valor: f64, data: chrono::NaiveDate, metodo_pagamento: MetodoPagamento) -> Self {
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
                self.data.format("%d/%m/%Y"), self.vendedor, self.codigo, self.valor, self.metodo_pagamento, self.produtos)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use super::*;
    
    #[test]
    fn test_categories_display() {
        assert_eq!(format!("{}", Categoria::Alimento), "Alimento");
        assert_eq!(format!("{}", Categoria::Eletronico), "Eletrônico");
        assert_eq!(format!("{}", Categoria::Geral), "Geral");
        assert_eq!(format!("{}", Categoria::Roupa), "Roupa");
    }

    #[test]
    fn test_payment_methods_display() {
        assert_eq!(format!("{}", MetodoPagamento::Credito), "Cartão de crédito");
        assert_eq!(format!("{}", MetodoPagamento::Debito), "Cartão de débito");
        assert_eq!(format!("{}", MetodoPagamento::Dinheiro), "Dinheiro");
        assert_eq!(format!("{}", MetodoPagamento::Pix), "PIX");
    }

    #[test]
    fn test_create_product() {
        let product = Produto::new("Smartphone".to_string(), 1, 100, 1500.0, 50, NaiveDate::default(), Categoria::Eletronico);

        assert_eq!(product.nome, "Smartphone");
        assert_eq!(product.id, 1);
        assert_eq!(product.quantidade_estoque, 100);
        assert_eq!(product.valor, 1500.0);
        assert_eq!(product.quantidade_restoque, 50);
        assert_eq!(product.data_restoque, NaiveDate::default());
    }

    #[test]
    fn test_product_display() {
        let product = Produto::new("Camisa".to_string(), 2, 50, 69.99, 10, NaiveDate::default(), Categoria::Roupa);

        let output = "Camisa\nID: 2\nEstoque: 50\nPreço: R$69.99\nMínimo para restoque: 10\nData do último restoque: 01/01/1970\nCategoria: Roupa";

        assert_eq!(format!("{product}"), format!("{output}"));
    }

    #[test]
    fn test_create_sale() {
        let venda = Venda::new("Lucas".to_string(), 2, 8.75, NaiveDate::default(), MetodoPagamento::Pix);

        assert_eq!(venda.vendedor, "Lucas");
        assert_eq!(venda.codigo, 2);
        assert_eq!(venda.valor, 8.75);
        assert_eq!(venda.data, NaiveDate::default());
    }

    #[test]
    fn test_sale_display() {
        let venda = Venda::new("Pedro".to_string(), 1, 100.50, NaiveDate::default(), MetodoPagamento::Debito);

        let output = "Data da venda: 01/01/1970\nVenda realizada por: Pedro\nCódigo: 1\nValor: R$100.50\nMétodo de pagamento: Cartão de débito\nProdutos vendidos:\n[]";

        assert_eq!(format!("{venda}"), format!("{output}"));
    }
}