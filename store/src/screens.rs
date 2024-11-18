pub fn menu_screen() {
    println!("\
Controle De Estoque
1 - Adicionar produtos
2 - Registrar venda
3 - Buscar produto por id
4 - Buscar produto por nome
5 - Emitir relatório de produtos com necessidade de restoque
6 - Buscar vendas por data
7 - Buscar vendas de um produto
8 - Atualizar caixa em serviço

Digite 'sair' para encerrar o programa");
}

pub fn add_product_screen() {
    println!("\
Insira as informações do produto no seguinte formato:
[Nome quantidade_em_estoque valor quantidade_para_restoque data_de_restoque(dd/mm/YYYY) categoria]

Digite 'sair' para cancelar a operação");
}

pub fn add_sale_screen() {
    println!("\
Insira o id e a quantidade dos produtos vendidos e digite 'concluir' para finalizar

Digite 'sair' para cancelar a operação.");
}
