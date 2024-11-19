use std::{error::Error, fs::File, io::{self, Seek, SeekFrom, Read, Write}};
use super::{errors, screens, validation, Produto, Venda};

const PRODUCT_LENGTH: usize = 102;

pub fn add_product(file: &mut File) -> Result<(), Box<dyn Error>> {
    let mut product;

    loop {
        screens::add_product_screen();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        if buf.trim().to_lowercase() == "sair" {
            return Ok(());
        }

        let fields: Vec<&str> = buf.split(' ').map(|field| field.trim()).collect();

        if fields.len() != 6 {
            eprintln!("Número incorreto de argumentos.");
            continue;
        }

        match validation::validate_product(fields) {
            Ok(value) => {
                product = value;
                break;
            },
            Err(error) => {
                eprintln!("Um erro ocorreu durante a conversão de argumentos: {error}.\nVerifique se todos os campos foram inseridos corretamente.");
                continue;
            }
        };
    }

    product.id = file.seek(SeekFrom::End(0))? / PRODUCT_LENGTH as u64 + 1;

    let mut serialized = bincode::serialize(&product)?;
    serialized.resize(PRODUCT_LENGTH, 0);
    file.write(&serialized)?;

    println!("Produto adicionado com sucesso com o id {}.", product.id);

    Ok(())
}

pub fn register_sale(products_file: &mut File, sales_file: &mut File, sales_index_file: &mut File, seller: String) -> Result<(), Box<dyn Error>> {
    
    screens::add_sale_screen();
    let mut products: Vec<(u64, u64)> = Vec::new();

    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        match buf.trim().to_lowercase().as_str() {
            "sair" => return Ok(()),
            "concluir" => break,
            _ => {}
        }

        products.push(validation::validate_sale(buf.trim())?);
    }

    let mut valor: f64 = 0.0;

    for info in products.iter() {
        let mut product = search_product_id(products_file, info.0)?;

        valor += product.valor * info.1 as f64;

        match info.1 < product.quantidade_estoque {
            true => return Err(Box::new(errors::CustomErrors::EmptyStock)),
            false => product.quantidade_estoque -= info.1
        }

        let position = PRODUCT_LENGTH as u64 * (info.0 - 1);
        products_file.seek(SeekFrom::Start(position))?;

        let mut serialized = bincode::serialize(&product)?;
        serialized.resize(PRODUCT_LENGTH, 0);
        products_file.write(&serialized)?;
    }

    let date = chrono::Local::now().date_naive();

    let mut sale = Venda::new(seller, 0, valor, date, validation::validate_payment_method()?);
    sale.codigo = sales_index_file.seek(SeekFrom::End(0))? / size_of::<(u64, u64)>() as u64 + 1;

    let serialized = bincode::serialize(&sale)?;

    let info = (sales_file.seek(SeekFrom::End(0))?, serialized.len() as u64);
    sales_file.write(&serialized)?;

    let serialized_index = bincode::serialize(&info)?;
    sales_index_file.write(&serialized_index)?;

    Ok(())
}

pub fn search_product_id(file: &mut File, id: u64) -> Result<Produto, Box<dyn Error>> {
    let position = PRODUCT_LENGTH as u64 * (id - 1);
    file.seek(SeekFrom::Start(position))?;

    let mut buf = [0; PRODUCT_LENGTH];
    file.read_exact(&mut buf)?;
    let product: Produto = bincode::deserialize(&buf)?;

    return Ok(product);
}

pub fn search_product_name(file: &mut File) -> Result<Produto, Box<dyn Error>> {
    println!("Digite o nome do produto que deseja procurar (ou sair para cancelar a operação):");
    
    let name = validation::validate_string()?;
    let mut buf = [0; PRODUCT_LENGTH];

    file.seek(SeekFrom::Start(0))?;

    loop {
        file.read_exact(&mut buf)?;

        let product: Produto = bincode::deserialize(&buf)?;

        if product.nome == name {
            println!("Produto encontrado.");
            return Ok(product);
        }
    }
}

pub fn products_needing_restock(file: &mut File) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; PRODUCT_LENGTH];
    let mut product: Produto;

    file.seek(SeekFrom::Start(0))?;

    while let Ok(_) = file.read_exact(&mut buf) {

        product = bincode::deserialize(&buf)?;

        if product.quantidade_estoque < product.quantidade_restoque {
            println!("{product}");
        }
    }

    Ok(())
}

pub fn search_sales_by_date(sales_file: &mut File, sales_index_file: &mut File, date: chrono::NaiveDate) -> Result<(), Box<dyn Error>> {
    sales_index_file.seek(SeekFrom::Start(0))?;
    let mut index_buf = [0; size_of::<(u64, u64)>()];

    while let Ok(_) = sales_index_file.read_exact(&mut index_buf) {
        let (position, size): (u64, u64) = bincode::deserialize(&index_buf)?;
        sales_file.seek(SeekFrom::Start(position))?;

        let mut buf = vec!(0; size as usize);
        sales_file.read_exact(&mut buf)?;
        let venda: Venda = bincode::deserialize(&buf)?;

        if venda.data == date {
            println!("{venda}");
        }
    }

    Ok(())
}

pub fn search_product_sales(sales_file: &mut File, sales_index_file: &mut File, id: u64) -> Result<(), Box<dyn Error>> {
    sales_index_file.seek(SeekFrom::Start(0))?;
    let mut index_buf = [0; size_of::<(u64, u64)>()];

    while let Ok(_) = sales_index_file.read_exact(&mut index_buf) {
        let (position, size): (u64, u64) = bincode::deserialize(&index_buf)?;
        sales_file.seek(SeekFrom::Start(position))?;

        let mut buf = vec!(0; size as usize);
        sales_file.read_exact(&mut buf)?;
        let venda: Venda = bincode::deserialize(&buf)?;

        if venda.produtos.contains(&id) {
            println!("{venda}");
        }
    }

    Ok(())
}

pub fn create_test_file(){
    
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

        let mut buf = [0; PRODUCT_LENGTH];
        let mut produto: Produto;

        for i in 1.. {
            let size_before = file.seek(SeekFrom::End(0)).expect("Erro no arquivo.");

            add_product(&mut file).unwrap_or_else(|error| {
                panic!("Um erro ocorreu ao tentar adicionar ao arquivo: {error}.");
            });

            let size_after = file.seek(SeekFrom::End(0)).expect("Erro no arquivo.");

            if size_before == size_after {
                break;
            }

            if size_after != PRODUCT_LENGTH as u64 * i {
                panic!("Produto não foi adicionado ao arquivo.");
            }

            file.seek(SeekFrom::Start(PRODUCT_LENGTH as u64 * (i - 1))).expect("Erro no arquivo.");
            file.read_exact(&mut buf).expect("Erro na leitura.");
            produto = bincode::deserialize(&buf).expect("Erro na desserialização.");

            if produto.id != i {
                panic!("Produto foi escrito incorretamente no arquivo.")
            }

            println!("{produto}");
        }
        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    }

    #[test]
    fn test_search_product_id(){
        let path = "test_searchId.bin";
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Não foi possível criar o arquivo.");
        
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
        let path = "test_add.bin";
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Não foi possível criar o arquivo.");
        
        let mut produto: Produto;
    

        produto = search_product_name(&mut file).unwrap_or_else(|error| {
            panic!("Um erro ocorreu ao buscar nome no arquivo: {error}.");
        });
    
            println!("{produto}");
        }
        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");
    
    #[test]
    fn test_products_needing_restock(){
        let path = "test_need_restock.bin";
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Não foi possível criar o arquivo.");
        
    
        products_needing_restock(&mut file).unwrap_or_else(|error| {
            panic!("Um erro ocorreu ao ler arquivo de produtos necessitando restoque: {error}.");
        });   

        println!("{produto}");
        }
        fs::remove_file(path).expect("Erro ao tentar excluir o arquivo.");

}
