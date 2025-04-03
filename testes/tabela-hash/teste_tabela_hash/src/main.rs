
use std::collections::HashMap;

// Função para criar a tabela hash
fn criar_tabela_hash() -> HashMap<String, Vec<String>> {
    let mut tabela_hash: HashMap<String, Vec<String>> = HashMap::new();
    tabela_hash.insert(String::from("Eletrônicos"), vec![String::from("Smartphone"), String::from("Laptop")]);
    tabela_hash.insert(String::from("Vestuário"), vec![String::from("Camiseta"), String::from("Calça")]);
    tabela_hash.insert(String::from("Decoração"), vec![String::from("Vaso"), String::from("Quadro")]);
    tabela_hash.insert(String::from("Alimentos"), vec![String::from("Chocolate"), String::from("Café")]);
    tabela_hash
}

// Função para exibir a tabela hash
fn exibir_tabela_hash(tabela_hash: &HashMap<String, Vec<String>>) {
    for (categoria, produtos) in tabela_hash.iter() {
        print!("Categoria: {} - Produtos: ", categoria);
        for (i, produto) in produtos.iter().enumerate() {
            if i == produtos.len() - 1 {
                print!("{}", produto); // Último produto, sem vírgula
            } else {
                print!("{}, ", produto); // Produtos com vírgula
            }
        }
        println!(); // Nova linha após cada categoria
    }
}

fn main() {
    let tabela_hash = criar_tabela_hash();
    exibir_tabela_hash(&tabela_hash);
}

// Testes unitários
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criar_tabela_hash() {
        let tabela_hash = criar_tabela_hash();
        assert_eq!(tabela_hash.len(), 4); // Verificar se há 4 categorias
        assert!(tabela_hash.contains_key("Eletrônicos"));
        assert!(tabela_hash.contains_key("Vestuário"));
        assert!(tabela_hash.contains_key("Decoração"));
        assert!(tabela_hash.contains_key("Alimentos"));

        // Verificar os produtos em uma categoria específica
        assert_eq!(tabela_hash.get("Eletrônicos").unwrap(), &vec!["Smartphone", "Laptop"]);
        assert_eq!(tabela_hash.get("Vestuário").unwrap(), &vec!["Camiseta", "Calça"]);
    }

    #[test]
    fn test_exibir_tabela_hash() {
        let tabela_hash = criar_tabela_hash();

        // Capturar a saída da função (simulação)
        let mut result = String::new();
        for (categoria, produtos) in tabela_hash.iter() {
            result.push_str(&format!("Categoria: {} - Produtos: ", categoria));
            for (i, produto) in produtos.iter().enumerate() {
                if i == produtos.len() - 1 {
                    result.push_str(produto); // Último produto, sem vírgula
                } else {
                    result.push_str(&format!("{}, ", produto)); // Produtos com vírgula
                }
            }
            result.push('\n'); // Nova linha após cada categoria
        }

        // Verificar se a saída contém categorias e produtos corretamente
        assert!(result.contains("Categoria: Eletrônicos - Produtos: Smartphone, Laptop"));
        assert!(result.contains("Categoria: Vestuário - Produtos: Camiseta, Calça"));
        assert!(result.contains("Categoria: Decoração - Produtos: Vaso, Quadro"));
        assert!(result.contains("Categoria: Alimentos - Produtos: Chocolate, Café"));
    }
}