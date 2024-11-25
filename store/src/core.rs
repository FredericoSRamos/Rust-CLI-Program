use std::{error::Error, fs::File, io::{BufRead, Read, Seek, SeekFrom, Write}};

use super::{errors, screens, validation, Produto, Venda};

const PRODUCT_LENGTH: usize = 102;
const PRODUCT_LENGTH_U64: u64 = 102;

pub fn add_product<R: BufRead>(file: &mut File, reader: &mut R) -> Result<(), Box<dyn Error>> {
    let mut product = validation::get_product_info(reader)?;

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

    println!("\nProduto adicionado com sucesso com o id {}.", product.id);

    Ok(())
}

pub fn register_sale<R: BufRead>(products_file: &mut File, sales_file: &mut File, seller: String, reader: &mut R) -> Result<(), Box<dyn Error>> {
    screens::add_sale_screen();
    let mut products: Vec<(u64, u64)> = Vec::new();

    loop {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        match buf.trim().to_lowercase().as_str() {
            "sair" => return Err(Box::new(errors::CustomErrors::OperationCanceled)),
            "concluir" => break,
            _ => ()
        }

        products.push(validation::validate_sale(buf.trim())?);
        println!("próximo produto na venda\n")
    }

    let mut value: f64 = 0.0;

    let mut sale = Venda::new(seller, 0, value, chrono::Local::now().date_naive(), validation::validate_payment_method(reader)?);

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

pub fn search_product_id(file: &mut File, id: u64) -> Result<(Produto, u64), Box<dyn Error>> {
    let mut left = 0;
    let mut right = file.seek(SeekFrom::End(-8))? / PRODUCT_LENGTH_U64;

    if right != 0 {
        right -= 1;
    }

    let mut buf = vec![0; PRODUCT_LENGTH];

    while left <= right {
        let mid= (left + right) / 2;
        file.seek(SeekFrom::Start(mid * PRODUCT_LENGTH_U64))?;
        file.read_exact(&mut buf)?;

        let product: Produto = bincode::deserialize(&buf)?;

        if product.id > id {
            if mid != 0 {
                right = mid - 1;
            }
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

        println!("\n--{product}");
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
            println!("\n--{product}");
        }
    }

    Ok(())
}

pub fn update_product<R: BufRead>(file: &mut File, reader: &mut R) -> Result<(), Box<dyn Error>> {
    let id = validation::validate_search("id", reader)?;
    let (product, position) = search_product_id(file, id)?;

    println!("Produto encontrado:\n\n{product}\n\n");
    let mut updated_product = validation::get_product_info(reader)?;
    updated_product.id = product.id;

    let serialized = bincode::serialize(&updated_product)?;
    file.seek(SeekFrom::Start(position))?;
    file.write(&serialized)?;

    Ok(())
}

pub fn remove_product<R: BufRead>(file: &mut File, reader: &mut R) -> Result<(), Box<dyn Error>> {
    let id = validation::validate_search("id", reader)?;
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
                println!("--{sale}\n");
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

pub fn update_sale<R: BufRead>(file: &mut File, reader: &mut R) -> Result<(), Box<dyn Error>> {
    let code = validation::validate_search("code", reader)?;
    let (mut sale, position) = search_sale_code(file, code)?;

    println!("Venda encontrada:\n\n{sale}\n\n");

    let (date, payment_method) = validation::get_sale_info(reader)?;
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

pub fn remove_sale<R: BufRead>(file: &mut File, reader: &mut R) -> Result<(), Box<dyn Error>> {
    let code = validation::validate_search("code", reader)?;
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

#[cfg(test)]
mod tests {
    use std::{fs::{self, OpenOptions}, io::Cursor};

    use super::*;

    fn get_test_file(path: &str) -> File {
        let _ = fs::remove_file(path);

        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Não foi possível criar o arquivo.")
    }

    fn set_products(file: &mut File) {
        let product1 = Produto::new("Teste1".to_string(), 1, 10, 50.0, 5, chrono::NaiveDate::default(), crate::Categoria::Geral);
        let product2 = Produto::new("Teste2".to_string(), 2, 15, 40.0, 25, chrono::NaiveDate::default(), crate::Categoria::Alimento);
        let product3 = Produto::new("Teste3".to_string(), 3, 20, 60.0, 10, chrono::NaiveDate::default(), crate::Categoria::Eletronico);

        let mut buf1 = bincode::serialize(&product1).unwrap();
        buf1.resize(102, 0);

        let mut buf2 = bincode::serialize(&product2).unwrap();
        buf2.resize(102, 0);

        let mut buf3 = bincode::serialize(&product3).unwrap();
        buf3.resize(102, 0);

        let id: u64 = 3;
        let serialized_id = bincode::serialize(&id).unwrap();

        file.write(&buf1).unwrap();
        file.write(&buf2).unwrap();
        file.write(&buf3).unwrap();

        file.write(&serialized_id).unwrap();
    }

    fn set_sales(file: &mut File) {
        let sale1 = Venda::new("Venda1".to_string(), 1, 50.0, chrono::NaiveDate::default(), crate::MetodoPagamento::Credito);
        let sale2 = Venda::new("Venda2".to_string(), 2, 70.0, chrono::NaiveDate::default(), crate::MetodoPagamento::Dinheiro);
        let mut sale3 = Venda::new("Venda3".to_string(), 3, 90.0, chrono::NaiveDate::default(), crate::MetodoPagamento::Pix);

        sale3.produtos.push(1);

        let buf1 = bincode::serialize(&sale1).unwrap();
        let size1 = buf1.len() as u64;
        let size_buf1 = bincode::serialize(&size1).unwrap();

        let buf2 = bincode::serialize(&sale2).unwrap();
        let size2 = buf2.len() as u64;
        let size_buf2 = bincode::serialize(&size2).unwrap();

        let buf3 = bincode::serialize(&sale3).unwrap();
        let size3 = buf3.len() as u64;
        let size_buf3 = bincode::serialize(&size3).unwrap();

        let code: u64 = 3;
        let serialized_code = bincode::serialize(&code).unwrap();

        file.write(&size_buf1).unwrap();
        file.write(&buf1).unwrap();
        file.write(&size_buf2).unwrap();
        file.write(&buf2).unwrap();
        file.write(&size_buf3).unwrap();
        file.write(&buf3).unwrap();

        file.write(&serialized_code).unwrap();
    }

    #[test]
    fn test_add_product() {
        let path = "test_add_product.bin";
        let mut file = get_test_file(path);

        let input = b"Camisa 10 50 5 10/8/2023 Geral";
        let mut cursor = Cursor::new(input);

        assert!(add_product(&mut file, &mut cursor).is_ok());

        let size = file.seek(SeekFrom::End(0)).expect("Erro no arquivo.");

        assert_eq!(size, PRODUCT_LENGTH_U64 + 8);

        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    }

    #[test]
    fn test_register_sale() {
        let path_products = "test_register_sale_1.bin";
        let path_sales = "test_register_sale_2.bin";

        let mut products_file = get_test_file(path_products);
        let mut sales_file = get_test_file(path_sales);

        set_products(&mut products_file);

        let products_size = products_file.seek(SeekFrom::End(0)).expect("Erro no arquivo de produtos.");
        let sales_size = sales_file.seek(SeekFrom::End(0)).expect("Erro no arquivo de vendas.");

        let input = "1\nconcluir\ndebito";
        let mut cursor = Cursor::new(input);

        let result = register_sale(&mut products_file, &mut sales_file, "Teste".to_string(), &mut cursor);

        assert!(result.is_ok());

        assert_eq!(products_size, products_file.seek(SeekFrom::End(0)).expect("Erro no arquivo de produtos."));
        assert!(sales_size < sales_file.seek(SeekFrom::End(0)).expect("Erro no arquivo de vendas."));

        fs::remove_file(path_products).expect("Erro ao tentar excluir o arquivo de produtos.");
        fs::remove_file(path_sales).expect("Erro ao tentar excluir o arquivo de vendas.");
    }

    #[test]
    fn test_search_product_id() {
        let path = "test_search_product_id.bin";
        let mut file = get_test_file(path);

        set_products(&mut file);

        let (found_product1, position1) = search_product_id(&mut file, 1).expect("Erro na busca pelo produto.");
        let (found_product2, position2) = search_product_id(&mut file, 2).expect("Erro na busca pelo produto.");
        let (found_product3, position3) = search_product_id(&mut file, 3).expect("Erro na busca pelo produto.");

        assert_eq!(found_product1.id, 1);
        assert_eq!(found_product2.id, 2);
        assert_eq!(found_product3.id, 3);

        assert_eq!(position1, 0);
        assert_eq!(position2, 102);
        assert_eq!(position3, 204);

        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    }

    #[test]
    fn test_list_products() {
        let path = "test_list_products.bin";
        let mut file = get_test_file(path);

        set_products(&mut file);

        assert!(list_products(&mut file).is_ok());

        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    }

    #[test]
    fn test_products_needing_restock() {
        let path = "test_products_needing_restock.bin";
        let mut file = get_test_file(path);

        set_products(&mut file);

        assert!(products_needing_restock(&mut file).is_ok());

        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    }

    #[test]
    fn test_update_product() {
        let path = "test_update_product.bin";
        let mut file = get_test_file(path);

        set_products(&mut file);

        let size = file.seek(SeekFrom::End(0)).expect("Erro no arquivo.");

        let input = b"1\nCamisa 10 50 5 10/8/2023 Geral";
        let mut cursor = Cursor::new(input);

        let result = update_product(&mut file, &mut cursor);

        assert!(result.is_ok());

        assert_eq!(size, file.seek(SeekFrom::End(0)).expect("Erro no arquivo."));

        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    }

    #[test]
    fn test_remove_product() {
        let path = "test_remove_product.bin";
        let mut file = get_test_file(path);

        set_products(&mut file);

        let size = file.seek(SeekFrom::End(0)).expect("Erro no arquivo.");

        let input = b"1";
        let mut cursor = Cursor::new(input);

        let result = remove_product(&mut file, &mut cursor);

        assert!(result.is_ok());

        assert_eq!(size - PRODUCT_LENGTH_U64, file.seek(SeekFrom::End(0)).expect("Erro no arquivo."));

        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    }

    #[test]
    fn test_search_sale_code() {
        let path = "test_search_sale_code.bin";
        let mut file = get_test_file(path);

        set_sales(&mut file);

        let result = search_sale_code(&mut file, 1);

        assert!(result.is_ok());

        let sale = result.unwrap();

        assert_eq!(sale.1, 0);
        assert_eq!(sale.0.codigo, 1);
        assert_eq!(sale.0.vendedor, "Venda1");
        assert_eq!(sale.0.valor, 50.0);

        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.")
    }

    #[test]
    fn test_search_sales_by_date() {
        let path = "test_search_sales_by_date.bin";
        let mut file = get_test_file(path);

        set_sales(&mut file);

        let result = search_sales_by_date(&mut file, chrono::NaiveDate::default());

        assert!(result.is_ok());

        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.")
    }

    #[test]
    fn test_search_product_sales() {
        let path = "test_search_product_sales.bin";
        let mut file = get_test_file(path);

        set_sales(&mut file);

        let result = search_product_sales(&mut file, 1);

        assert!(result.is_ok());

        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.")
    }

    #[test]
    fn test_list_sales() {
        let path = "test_list_sales.bin";
        let mut file = get_test_file(path);

        set_sales(&mut file);

        let result = list_sales(&mut file);

        assert!(result.is_ok());

        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    }

    #[test]
    fn test_update_sale() {
        let path = "test_update_sale.bin";
        let mut file = get_test_file(path);

        set_sales(&mut file);

        let input = b"2\n1/8/2023\npix";
        let mut cursor = Cursor::new(input);

        let size = file.seek(SeekFrom::End(0)).expect("Erro no arquivo.");

        let result = update_sale(&mut file, &mut cursor);

        assert!(result.is_ok());

        assert_eq!(size, file.seek(SeekFrom::End(0)).expect("Erro no arquivo."));

        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    }

    #[test]
    fn test_remove_sale() {
        let path = "test_remove_sale.bin";
        let mut file = get_test_file(path);

        set_sales(&mut file);

        let input = b"1";
        let mut cursor = Cursor::new(input);

        let size = file.seek(SeekFrom::End(0)).expect("Erro no arquivo.");

        let result = remove_sale(&mut file, &mut cursor);

        assert!(result.is_ok());

        assert!(size > file.seek(SeekFrom::End(0)).expect("Erro no arquivo."));

        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    }
}