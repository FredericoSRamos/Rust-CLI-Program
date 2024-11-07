use std::{fs::File, process};
use chrono::{self, NaiveDate};

extern crate store;

fn main() {
    println!("Hello, world!");

    let mut buf = String::new();

    let mut file = File::open(store::validation::validate_path(&mut buf)).unwrap_or_else(|error| {
        eprintln!("Ocorreu um erro tentando abrir o arquivo: {error}.");
        process::exit(1);
    });

    let mut produto = store::Produto::default();
    let mut id: u64;
    let mut data: chrono::NaiveDate;

    loop {
        let option = match store::gather_option() {
            Ok(option) => option,
            Err(error) => {
                eprintln!("Ocorreu um erro ao tentar pegar a opção selecionada: {error}.\nCertifique-se de ter digitado corretamente a opção desejada");
                continue;
            }
        };

        let result = match option {
            0 => {
                process::exit(0);
            }
            1 => store::add_product(&mut file),
            2 => store::register_sale(&mut file),
            3 => {
                id = store::validation::validate_id_search();
                store::search_id(&mut file, &mut id, &mut produto)
            }
            4 => store::products_needing_restock(&mut file),
            5 => { 
                data = store::validation::validade_data_search();
                store::search_sale_data(&mut file, &mut data,&mut produto)
            }
            6 => store::search_sale_product(&mut file)
            _ => {
                eprintln!("Insira um valor válido de operação");
                continue;
            }
        };

        if let Err(error) = result {
            eprintln!("Um erro ocorreu durante a operação: {error}.");
        }
    }
}
