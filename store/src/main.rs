use store::{core, validation};

extern crate store;

fn main() {

    let (mut products_file, mut sales_file) = validation::get_files();

    println!("Insira o caixa que está realizando as vendas:");
    let mut seller = validation::get_string();

    loop {
        let result = match validation::get_option() {
            0 => return,
            1 => core::add_product(&mut products_file),
            2 => core::register_sale(&mut products_file, &mut sales_file, seller.clone()),
            3 => match validation::validate_search("id") {
                Ok(id) => match core::search_product_id(&mut products_file, id) {
                    Ok((product, _)) => {
                        println!("{product}");
                        Ok(())
                    },
                    Err(error) => Err(error)
                },
                Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            4 => core::list_products(&mut products_file),
            5 => core::products_needing_restock(&mut products_file),
            6 => core::update_product(&mut products_file),
            7 => core::remove_product(&mut products_file),
            8 => match validation::validate_search("code") {
                Ok(code) => match core::search_sale_code(&mut sales_file, code) {
                    Ok((sale, _)) => {
                        println!("{sale}\n");
                        Ok(())
                    },
                    Err(error) => Err(error)
                },
                Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>)
            },
            9 => {
                println!("Digite a data da venda que deseja procurar seguindo o formato dd/mm/YYYY ou digite 'sair' para cancelar");
                match validation::validate_date() {
                    Ok(date) => core::search_sales_by_date(&mut sales_file, date),
                    Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>)
                }
            },
            10 => match validation::validate_search("id") {
                Ok(id) => core::search_product_sales(&mut sales_file, id),
                Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>)
            },
            11 => core::list_sales(&mut sales_file),
            12 => core::update_sale(&mut sales_file),
            13 => core::remove_sale(&mut sales_file),
            14 => {
                println!("Insira o caixa que está realizando as vendas:");
                seller = validation::get_string();

                Ok(())
            },
            _ => {
                eprintln!("Insira um valor válido de operação.");

                Ok(())
            }
        };

        if let Err(error) = result {
            if let Some(custom) = error.downcast_ref::<store::errors::CustomErrors>() {
                match custom {
                    store::errors::CustomErrors::OperationCanceled => (),
                    _ => eprintln!("Um erro ocorreu durante a operação: {error}")
                }
            } else {
                eprintln!("Um erro ocorreu durante a operação: {error}");
            }
        }
    }
}