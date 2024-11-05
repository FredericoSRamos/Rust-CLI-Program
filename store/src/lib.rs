use chrono;
pub enum Categoria {
    Eletronico,
    Roupa,
    Alimento
}

pub struct Produto {
    pub nome: String,
    pub id: String,
    pub quantidade_estoque: i64,
    pub valor: f64,
    pub quantidade_restoque: i64,
    pub data_restoque: chrono::NaiveDate,
    pub categoria: Categoria
}

impl Produto {
    pub fn new(nome: String, id: String, quantidade_estoque: i64, valor: f64, quantidade_restoque: i64, data_restoque: chrono::NaiveDate, categoria: Categoria) -> Self {
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