pub fn menu_screen() {
    println!("\
    \n\n-----------------------------
      \n\nControle De Estoque
    
    1  -  Adicionar produtos
    2  -  Registrar venda
    3  -  Buscar produto por id
    4  -  Listar produtos
    5  -  Emitir relatório de produtos com necessidade de restoque
    6  -  Atualizar produto
    7  -  Remover produto
    8  -  Buscar venda por código
    9  -  Buscar vendas por data
    10 -  Buscar vendas de um produto
    11 -  Listar vendas
    12 -  Atualizar venda
    13 -  Remover venda
    14 -  Atualizar caixa em serviço

    Digite 'sair' para encerrar o programa
    \n\n-----------------------------");
}

pub fn add_product_screen() {
    println!("\
  \n\nInsira as informações do produto no seguinte formato:
  \n* Atenção: insira os campos com um espaço entre eles! *\n
  Exemplo de input: (lapis 3 200.00 5 20/05/2024 alimento)\n
  [Nome, quantidade em estoque, valor, quantidade minima para que seja necessitado um restoque, data do ultimo restoque no seguinte formato: dd/mm/YYYY *categoria*]
  \n* Categorias possíveis * : eletronico, roupa, alimento, geral
  * Atenção: Não utilizar acento em categoria! *\n

  Digite 'sair' para cancelar a operação\n\n");
}

pub fn add_sale_screen() {
    println!("\
Insira o id e a quantidade dos produtos vendidos e digite 'concluir' para finalizar

Digite 'sair' para cancelar a operação.");
}
