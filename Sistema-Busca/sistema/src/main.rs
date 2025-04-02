use std::collections::{HashMap, HashSet};

// Estrutura do catálogo com suporte a pré-processamento e cache
struct Catalogo {
    tabela_hash: HashMap<String, Vec<String>>,
    cache: HashMap<String, Vec<String>>,
    buscas_frequentes: HashSet<String>, // Para rastrear buscas populares
    grafo: HashMap<String, Vec<String>>, // Grafo para recomendações de produtos
}

impl Catalogo {
    // Função para criar um novo catálogo
    fn novo() -> Self {
        let mut tabela_hash = HashMap::new();
        tabela_hash.insert(String::from("Eletrônicos"), vec![String::from("Smartphone"), String::from("Laptop")]);
        tabela_hash.insert(String::from("Vestuário"), vec![String::from("Camiseta"), String::from("Calça")]);
        tabela_hash.insert(String::from("Decoração"), vec![String::from("Vaso"), String::from("Quadro")]);
        tabela_hash.insert(String::from("Alimentos"), vec![String::from("Chocolate"), String::from("Café")]);

        let mut grafo = HashMap::new();
        grafo.insert(String::from("Smartphone"), vec![String::from("Capa de Smartphone"), String::from("Carregador")]);
        grafo.insert(String::from("Laptop"), vec![String::from("Mouse"), String::from("Mochila para Laptop")]);
        grafo.insert(String::from("Camiseta"), vec![String::from("Calça"), String::from("Tênis")]);
        grafo.insert(String::from("Vaso"), vec![String::from("Plantas"), String::from("Terra Adubada")]);

        Catalogo {
            tabela_hash,
            cache: HashMap::new(),
            buscas_frequentes: HashSet::new(),
            grafo,
        }
    }

    // Função para remover stop words
    fn remover_stop_words(&self, texto: &str, stop_words: &[&str]) -> String {
        texto.split_whitespace()
            .filter(|palavra| !stop_words.contains(palavra))
            .collect::<Vec<&str>>()
            .join(" ")
    }

    // Função para simular stemming
    fn aplicar_stemming(&self, texto: &str) -> String {
        texto.to_lowercase()
            .replace("s", "")
            .replace("ção", "co")
    }

    // Função para corrigir erros de ortografia
    fn corrigir_ortografia(&self, texto: &str) -> String {
        if texto.to_lowercase() == "eletronicos" {
            "Eletrônicos".to_string()
        } else {
            texto.to_string()
        }
    }

    // Função para buscar produtos por categoria
    fn buscar_por_categoria(&mut self, termo_busca: &str) {
        let stop_words = ["de", "e", "o", "a", "os", "as"];
        let termo_corrigido = self.corrigir_ortografia(termo_busca);
        let termo_sem_stop_words = self.remover_stop_words(&termo_corrigido, &stop_words);
        let termo_normalizado = self.aplicar_stemming(&termo_sem_stop_words);

        if let Some(produtos) = self.cache.get(&termo_normalizado) {
            println!("(Cache) Produtos encontrados na categoria '{}': {}", termo_busca, produtos.join(", "));
            return;
        }

        if let Some(produtos) = self.tabela_hash.get(&termo_normalizado) {
            println!("Produtos encontrados na categoria '{}': {}", termo_busca, produtos.join(", "));
            self.cache.insert(termo_normalizado.to_string(), produtos.clone());
            self.registrar_busca(&termo_normalizado);
        } else {
            println!("Categoria '{}' não encontrada.", termo_busca);
        }
    }

    // Função para registrar buscas frequentes
    fn registrar_busca(&mut self, categoria: &str) {
        self.buscas_frequentes.insert(categoria.to_string());
    }

    // Função para listar buscas frequentes
    fn listar_buscas_frequentes(&self) {
        if self.buscas_frequentes.is_empty() {
            println!("Nenhuma busca frequente registrada.");
        } else {
            println!("Buscas frequentes: {:?}", self.buscas_frequentes);
        }
    }

    // Função para listar todas as categorias e produtos
    fn listar_todos(&self) {
        for (categoria, produtos) in &self.tabela_hash {
            println!("Categoria: {} - Produtos: {}", categoria, produtos.join(", "));
        }
    }

    // Função para exibir recomendações de produtos
    fn exibir_recomendacoes(&self, produto: &str) {
        if let Some(recomendacoes) = self.grafo.get(produto) {
            println!("Recomendações para '{}': {}", produto, recomendacoes.join(", "));
        } else {
            println!("Nenhuma recomendação encontrada para '{}'.", produto);
        }
    }
}

fn main() {
    let mut catalogo = Catalogo::novo();

    // Listando todas as categorias e produtos
    catalogo.listar_todos();

    // Realizando buscas
    catalogo.buscar_por_categoria("eletronicos");
    catalogo.buscar_por_categoria("alimentos");
    catalogo.buscar_por_categoria("eletronicos");

    // Listando buscas frequentes
    catalogo.listar_buscas_frequentes();

    // Exibindo recomendações de produtos
    catalogo.exibir_recomendacoes("Smartphone");
    catalogo.exibir_recomendacoes("Laptop");
}