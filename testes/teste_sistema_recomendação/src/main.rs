use std::collections::HashMap;

fn main() {
    // Representação do grafo de recomendação
    let mut grafo: HashMap<String, Vec<String>> = HashMap::new();

    // Relacionando produtos (arestas)
    grafo.insert(String::from("Smartphone"), vec![String::from("Capa de Smartphone"), String::from("Carregador")]);
    grafo.insert(String::from("Laptop"), vec![String::from("Mouse"), String::from("Mochila para Laptop")]);
    grafo.insert(String::from("Camiseta"), vec![String::from("Calça"), String::from("Tênis")]);
    grafo.insert(String::from("Vaso"), vec![String::from("Plantas"), String::from("Terra Adubada")]);

    // Função para exibir recomendações
    let produto = "Smartphone";
    if let Some(recomendacoes) = grafo.get(produto) {
        println!("Recomendações para '{}': {}", produto, recomendacoes.join(", "));
    } else {
        println!("Nenhuma recomendação encontrada para '{}'.", produto);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recomendacoes_para_smartphone() {
        let mut grafo: HashMap<String, Vec<String>> = HashMap::new();
        grafo.insert(String::from("Smartphone"), vec![String::from("Capa de Smartphone"), String::from("Carregador")]);
        grafo.insert(String::from("Laptop"), vec![String::from("Mouse"), String::from("Mochila para Laptop")]);
        grafo.insert(String::from("Camiseta"), vec![String::from("Calça"), String::from("Tênis")]);
        grafo.insert(String::from("Vaso"), vec![String::from("Plantas"), String::from("Terra Adubada")]);

        let produto = "Smartphone";
        if let Some(recomendacoes) = grafo.get(produto) {
            assert_eq!(recomendacoes, &vec![String::from("Capa de Smartphone"), String::from("Carregador")]);
        } else {
            panic!("Nenhuma recomendação encontrada para '{}'.", produto);
        }
    }

    #[test]
    fn test_produto_sem_recomendacoes() {
        let mut grafo: HashMap<String, Vec<String>> = HashMap::new();
        grafo.insert(String::from("Smartphone"), vec![String::from("Capa de Smartphone"), String::from("Carregador")]);

        let produto = "Tablet"; // Produto que não existe no grafo
        let recomendacoes = grafo.get(produto);
        assert!(recomendacoes.is_none());
    }
}