use std::{error::Error, fs::File, io::{self, Seek, SeekFrom, Read, Write}};

use super::{errors, screens, validation, Produto, Venda};

const PRODUCT_LENGTH: usize = 102;
const PRODUCT_LENGTH_U64: u64 = 102;

pub fn add_product(file: &mut File) -> Result<(), Box<dyn Error>> {
    let mut product = validation::get_product_info()?;

    if file.seek(SeekFrom::End(0))? == 0 {
        product.id = 1;
    } else {
        let mut buf = vec![0; 8];

        file.seek(SeekFrom::End(-8))?;
        file.read_exact(&mut buf)?;

        let id: u64 = bincode::deserialize(&buf)?;
        product.id = id + 1;

        file.seek(SeekFrom::End(-8))?;
    }

    let mut serialized = bincode::serialize(&product)?;
    serialized.resize(PRODUCT_LENGTH, 0);
    file.write(&serialized)?;

    let id = product.id;
    let serialized_id = bincode::serialize(&id)?;
    file.write(&serialized_id)?;

    println!("Produto adicionado com sucesso com o id {}.", product.id);

    Ok(())
}

pub fn update_product(file: &mut File) -> Result<(), Box<dyn Error>> {
    let id = validation::validate_search("id")?;
    let (product, position) = search_product_id(file, id)?;

    println!("Produto encontrado:\n\n{product}\n\n");
    let mut updated_product = validation::get_product_info()?;
    updated_product.id = product.id;

    let serialized = bincode::serialize(&updated_product)?;
    file.seek(SeekFrom::Start(position))?;
    file.write(&serialized)?;

    Ok(())
}

pub fn remove_product(file: &mut File) -> Result<(), Box<dyn Error>> {
    let id = validation::validate_search("id")?;
    let (_, mut position) = search_product_id(file, id)?;

    let size = file.seek(SeekFrom::End(0))? - PRODUCT_LENGTH_U64;
    file.seek(SeekFrom::Start(position))?;

    loop {
        let mut buf = vec![0; PRODUCT_LENGTH * 100];
        file.seek(SeekFrom::Current(PRODUCT_LENGTH_U64 as i64))?;
        let bytes_read = file.read(&mut buf)?;
        file.seek(SeekFrom::Start(position))?;
        file.write(&buf[..bytes_read])?;
        position = file.seek(SeekFrom::Current(0))?;

        if bytes_read != PRODUCT_LENGTH * 100 {
            file.set_len(size)?;
            return Ok(());
        }
    }
}

pub fn search_product_id(file: &mut File, id: u64) -> Result<(Produto, u64), Box<dyn Error>> {
    let mut left = 0;
    let mut right = file.seek(SeekFrom::End(-8))? / PRODUCT_LENGTH_U64 - 1;
    let mut buf = vec![0; PRODUCT_LENGTH];

    while left <= right {
        let mid= (left + right) / 2;
        file.seek(SeekFrom::Start(mid * PRODUCT_LENGTH_U64))?;
        file.read_exact(&mut buf)?;

        let product: Produto = bincode::deserialize(&buf)?;

        if product.id > id && mid != 0 {
            right = mid - 1;
        } else if product.id < id {
            left = mid + 1;
        } else {
            return Ok((product, mid * PRODUCT_LENGTH_U64));
        }
    }
    
    Err(Box::new(errors::CustomErrors::ProductNotFound))
}

pub fn list_products(file: &mut File) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; PRODUCT_LENGTH];
    let mut product: Produto;

    file.seek(SeekFrom::Start(0))?;

    while let Ok(_) = file.read_exact(&mut buf) {

        product = bincode::deserialize(&buf)?;

        println!("{product}");
    }

    Ok(())
}

pub fn products_needing_restock(file: &mut File) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; PRODUCT_LENGTH];
    let mut product: Produto;

    file.seek(SeekFrom::Start(0))?;

    while let Ok(_) = file.read_exact(&mut buf) {

        product = bincode::deserialize(&buf)?;

        if product.quantidade_estoque <= product.quantidade_restoque {
            println!("{product}");
        }
    }

    Ok(())
}

pub fn register_sale(products_file: &mut File, sales_file: &mut File, seller: String) -> Result<(), Box<dyn Error>> {
    screens::add_sale_screen();
    let mut products: Vec<(u64, u64)> = Vec::new();

    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        match buf.trim().to_lowercase().as_str() {
            "sair" => return Ok(()),
            "concluir" => break,
            _ => ()
        }

        products.push(validation::validate_sale(buf.trim())?);
    }

    let mut value: f64 = 0.0;

    let mut sale = Venda::new(seller, 0, value, chrono::Local::now().date_naive(), validation::validate_payment_method()?);

    for &(id, amount) in products.iter() {
        let (mut product, position) = search_product_id(products_file, id)?;

        value += product.valor * amount as f64;

        match amount > product.quantidade_estoque {
            true => return Err(Box::new(errors::CustomErrors::LowStock)),
            false => product.quantidade_estoque -= amount
        }

        products_file.seek(SeekFrom::Start(position))?;

        let mut serialized = bincode::serialize(&product)?;
        serialized.resize(PRODUCT_LENGTH, 0);
        products_file.write(&serialized)?;

        if !sale.produtos.contains(&product.id) {
            sale.produtos.push(product.id);
        }
    }

    sale.valor = value;

    if sales_file.seek(SeekFrom::End(0))? == 0 {
        sale.codigo = 1;
    } else {
        sales_file.seek(SeekFrom::End(-8))?;

        let mut buf = vec![0; 8];
        sales_file.read_exact(&mut buf)?;

        let code: u64 = bincode::deserialize(&buf)?;

        sale.codigo = code + 1;
        sales_file.seek(SeekFrom::End(-8))?;
    }

    let serialized = bincode::serialize(&sale)?;
    let size = serialized.len() as u64;
    let code = sale.codigo;
    let serialized_size = bincode::serialize(&size)?;
    let serialized_code = bincode::serialize(&code)?;

    sales_file.write(&serialized_size)?;
    sales_file.write(&serialized)?;
    sales_file.write(&serialized_code)?;

    Ok(())
}

pub fn update_sale(file: &mut File) -> Result<(), Box<dyn Error>> {
    let code = validation::validate_search("code")?;
    let (mut sale, position) = search_sale_code(file, code)?;

    println!("Venda encontrada:\n\n{sale}\n\n");

    let (date, payment_method) = validation::get_sale_info()?;
    sale.data = date;
    sale.metodo_pagamento = payment_method;

    let serialized = bincode::serialize(&sale)?;
    let size = serialized.len() as u64;
    let serialized_size = bincode::serialize(&size)?;

    file.seek(SeekFrom::Start(position))?;
    file.write(&serialized_size)?;
    file.write(&serialized)?;

    Ok(())
}

pub fn remove_sale(file: &mut File) -> Result<(), Box<dyn Error>> {
    let code = validation::validate_search("code")?;
    let (sale, mut position) = search_sale_code(file, code)?;

    let size = bincode::serialized_size(&sale)?;
    file.seek(SeekFrom::Start(position))?;

    loop {
        let mut buf = vec![0; 100000];
        file.seek(SeekFrom::Current(size as i64 + 8))?;
        let bytes_read = file.read(&mut buf)?;
        file.seek(SeekFrom::Start(position))?;
        file.write(&buf[..bytes_read])?;
        position = file.seek(SeekFrom::Current(0))?;

        if bytes_read != 100000 {
            file.set_len(position)?;
            return Ok(())
        }
    }
}

pub fn search_sale_code(file: &mut File, code: u64) -> Result<(Venda, u64), Box<dyn Error>> {
    file.seek(SeekFrom::Start(0))?;
    let mut size_buf = vec![0; 8];

    while let Ok(_) = file.read_exact(&mut size_buf) {
        let size: u64 = bincode::deserialize(&size_buf)?;
        let mut buf = vec![0; size as usize];

        if let Ok(_) = file.read_exact(&mut buf) {
            let sale: Venda = bincode::deserialize(&buf)?;
            
            if sale.codigo == code {
                let position = file.seek(SeekFrom::Current(-((size_buf.len() + buf.len()) as i64)))?;
                return Ok((sale, position));
            }
        }
    }

    Err(Box::new(errors::CustomErrors::SaleNotFound))
}

pub fn search_sales_by_date(file: &mut File, date: chrono::NaiveDate) -> Result<(), Box<dyn Error>> {
    file.seek(SeekFrom::Start(0))?;
    let mut size_buf = vec![0; 8];

    println!("Vendas realizadas na data especificada:\n");

    while let Ok(_) = file.read_exact(&mut size_buf) {
        let size: u64 = bincode::deserialize(&size_buf)?;
        let mut buf = vec![0; size as usize];

        if let Ok(_) = file.read_exact(&mut buf) {
            let sale: Venda = bincode::deserialize(&buf)?;

            if sale.data == date {
                println!("{sale}\n");
            }
        }
    }

    Ok(())
}

pub fn search_product_sales(file: &mut File, id: u64) -> Result<(), Box<dyn Error>> {
    file.seek(SeekFrom::Start(0))?;
    let mut size_buf = vec![0; 8];

    println!("Vendas do produto especificado:\n");

    while let Ok(_) = file.read_exact(&mut size_buf) {
        let size: u64 = bincode::deserialize(&size_buf)?;
        let mut buf = vec![0; size as usize];

        if let Ok(_) = file.read_exact(&mut buf) {
            let sale: Venda = bincode::deserialize(&buf)?;

            if sale.produtos.contains(&id) {
                println!("{sale}\n");
            }
        }
    }

    Ok(())
}

pub fn list_sales(file: &mut File) -> Result<(), Box<dyn Error>> {
    file.seek(SeekFrom::Start(0))?;
    let mut size_buf = vec![0; 8];

    while let Ok(_) = file.read_exact(&mut size_buf) {
        let size: u64 = bincode::deserialize(&size_buf)?;
        let mut buf = vec![0; size as usize];

        if let Ok(_) = file.read_exact(&mut buf) {
            let sale: Venda = bincode::deserialize(&buf)?;
            println!("{sale}\n");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, OpenOptions};

    #[test]
    fn test_add_product() {
        let path = "test_add.bin";

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Não foi possível criar o arquivo.");

        loop {
            let size_before = file.seek(SeekFrom::End(0)).expect("Erro no arquivo.");

            let _ = add_product(&mut file);

            let size_after = file.seek(SeekFrom::End(0)).expect("Erro no arquivo.");

            if size_before == size_after {
                break;
            }

            if size_before == 0 {
                assert_eq!(size_before + 8, size_after - PRODUCT_LENGTH_U64);
            } else {
                assert_eq!(size_before, size_after - PRODUCT_LENGTH_U64);
            }
        }

        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    }

   /* #[test]
    fn test_search_product_id(){
        
        let file = create_test_file();
        let path = "test_file.bin"
        
        let mut produto: Produto;

        let size_file = file.seek(SeekFrom::End(0)).expect("Erro no arquivo.");

        for i .. size_file  { 
    
            produto = search_product_id(&mut file, i).unwrap_or_else(|error| {
                panic!("Um erro ocorreu ao buscar id no arquivo: {error}.");
            });
    
            file.seek(SeekFrom::Start(PRODUCT_LENGTH as u64 * (i - 1))).expect("Erro no arquivo.");
    
            println!("{produto}");
        }
        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");

    }

    #[test]
    fn test_search_product_name(){

        let mut file = create_test_file();
        let path = "test_file.bin"
        
        let mut produto: Produto;
    

        produto = search_product_name(&mut file).unwrap_or_else(|error| {
            panic!("Um erro ocorreu ao buscar nome no arquivo: {error}.");
        });
    
            println!("{produto}");
        
        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    }
    #[test]
    fn test_products_needing_restock(){
        
        let mut file = create_test_file();
        let path = "test_file.bin"
    
        products_needing_restock(&mut file).unwrap_or_else(|error| {
            panic!("Um erro ocorreu ao ler arquivo de produtos necessitando restoque: {error}.");
        });   

        println!("{produto}");
        
        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    }*/
}
