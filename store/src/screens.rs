pub fn menu_screen() {

    println!("\
Controle De Estoque
1 - Adicionar produtos
2 - Registrar venda
3 - Buscar produto por id
4 - Emitir relatório de produtos com necessidade de restoque
5 - Buscar vendas por data
6 - Emitir relatório de vendas de um produto 
Digite 'sair' para cancelar");

}

pub fn add_screen() {
    println!("\
Insira a descrição do produto com os campos separados por ',' ou insira 'sair' para cancelar
[Nome, quantidade_em_estoque, valor, quantidade_para_restoque, data_de_restoque (dd/mm/YYYY), categoria]");
}

pub fn add_sale_screen() {
    println!("\
Insira a descrição da venda com os campos separados por ',' ou insira 'sair' para cancelar
[Produto, numero_produtos, valor, data_de_restoque (dd/mm/YYYY), metodo_pagamento]");
}

