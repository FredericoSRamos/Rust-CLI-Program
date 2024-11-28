use std::{io::stdin, process};

use store::{core, validation};

extern crate store;

fn main() {

    let (mut products_file, mut sales_file) = validation::get_files();

    println!("\nInsira o nome do caixa que está realizando as vendas (ou 'sair' para encerrar a aplicação):");
    let mut seller = validation::validate_string(&mut stdin().lock()).unwrap_or_else(|_| {
        process::exit(0);
    });

    loop {
        let result = match validation::get_option() {
            0 => process::exit(0),
            1 => core::add_product(&mut products_file, &mut std::io::stdin().lock()),
            2 => core::register_sale(&mut products_file, &mut sales_file, seller.clone(), &mut stdin().lock()),
            3 => match validation::validate_search("id", &mut stdin().lock()) {
                Ok(id) => match core::search_product_id(&mut products_file, id) {
                    Ok((product, _)) => {
                        println!("\n{product}\n");
                        Ok(())
                    },
                    Err(error) => Err(error)
                },
                Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            4 => core::list_products(&mut products_file),
            5 => core::products_needing_restock(&mut products_file),
            6 => core::update_product(&mut products_file, &mut stdin().lock()),
            7 => core::remove_product(&mut products_file, &mut stdin().lock()),
            8 => match validation::validate_search("code", &mut stdin().lock()) {
                Ok(code) => match core::search_sale_code(&mut sales_file, code) {
                    Ok((sale, _)) => {
                        println!("\n{sale}\n");
                        Ok(())
                    },
                    Err(error) => Err(error)
                },
                Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>)
            },
            9 => {
                println!("\nDigite a data da venda que deseja procurar seguindo o formato dd/mm/YYYY (ou digite 'sair' para cancelar):");
                match validation::validate_date(&mut stdin().lock()) {
                    Ok(date) => core::search_sales_by_date(&mut sales_file, date),
                    Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>)
                }
            },
            10 => match validation::validate_search("id", &mut stdin().lock()) {
                Ok(id) => core::search_product_sales(&mut sales_file, id),
                Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>)
            },
            11 => core::list_sales(&mut sales_file),
            12 => core::update_sale(&mut sales_file, &mut stdin().lock()),
            13 => core::remove_sale(&mut sales_file, &mut stdin().lock()),
            14 => {
                println!("\nInsira o nome do caixa que está realizando as vendas (ou 'sair' para cancelar):");

                if let Ok(name) = validation::validate_string(&mut stdin().lock()) {
                    seller = name;
                }

                Ok(())
            },
            _ => {
                eprintln!("\nInsira um valor válido de operação.\n");

                Ok(())
            }
        };

        if let Err(error) = result {
            if let Some(custom) = error.downcast_ref::<store::errors::CustomErrors>() {
                match custom {
                    store::errors::CustomErrors::OperationCanceled => (),
                    _ => eprintln!("\nUm erro ocorreu durante a operação: {error}\n")
                }
            } else {
                eprintln!("\nUm erro ocorreu durante a operação: {error}\n");
            }
        }
    }
}