use std::{fs::File, process};

extern crate store;

fn main() {
    let mut file = File::open(store::validation::get_path()).unwrap_or_else(|error| {
        eprintln!("Ocorreu um erro tentando abrir o arquivo: {error}.");
        process::exit(1);
    });

    let mut produto = store::Produto::default();

    loop {
        let option = match store::get_option() {
            Ok(option) => option,
            Err(error) => {
                eprintln!("Ocorreu um erro ao tentar pegar a opção selecionada: {error}.\nCertifique-se de ter digitado corretamente a opção desejada");
                continue;
            }
        };

        let result = match option {
            0 => process::exit(0),
            1 => store::add_product(&mut file),
            2 => store::register_sale(&mut file),
            3 => store::search_id(&mut file, store::validation::validate_id_search(), &mut produto),
            4 => store::search_product(&mut file, &mut produto),
            5 => store::products_needing_restock(&mut file),
            6 => store::search_sale_date(&mut file, store::validation::validate_date()),
            7 => store::search_product_sales(&mut file),
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
