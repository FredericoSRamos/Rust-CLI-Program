use store::{core, validation};

extern crate store;

fn main() {

    let (mut products_file, mut sales_file, mut sales_index_file) = validation::get_files();

    let mut produto = store::Produto::default();

    println!("Insira o caixa que está realizando as vendas:");
    let mut seller = validation::get_string();

    loop {
        let result = match validation::get_option() {
            0 => return,
            1 => core::add_product(&mut products_file),
            2 => core::register_sale(&mut products_file, &mut sales_file, &mut sales_index_file, seller.clone()),
            3 => match validation::validate_id_search() {
                Ok(id) => core::search_product_id(&mut products_file, id, &mut produto),
                Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>)
            },
            4 => {
                println!("Digite o nome do produto que deseja procurar (ou sair para cancelar a operação):");

                match validation::validate_string() {
                    Ok(string) => core::search_product_name(&mut products_file, string, &mut produto),
                    Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>)
                }
            },
            5 => core::products_needing_restock(&mut products_file),
            6 => match validation::validate_date() {
                Ok(date) => core::search_sales_by_date(&mut sales_file, &mut sales_index_file, date),
                Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>)
            },
            7 => match validation::validate_id_search() {
                Ok(id) => core::search_product_sales(&mut sales_file, &mut sales_index_file, id),
                Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>)
            },
            8 => {
                println!("Insira o caixa que está realizando as vendas:");
                seller = validation::get_string();

                Ok(())
            },
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
